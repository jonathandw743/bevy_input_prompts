use anyhow::{Result, anyhow};
use fixedbitset::FixedBitSet;
use hashbrown::HashMap;
use indexmap::IndexSet;
use itertools::Itertools;
use quote::{format_ident, quote};
use std::{
    borrow::Cow,
    collections::{BTreeSet, HashSet, VecDeque, vec_deque},
    fmt::Debug,
    iter::once,
    ops::Range,
    path::{Component, Path, PathBuf},
    rc::Rc,
};
use syn::{
    Index, LitInt, LitStr, Token,
    parse::{Parse, ParseStream},
};
use walkdir::WalkDir;
// use proc_macro2::{TokenStream, TokenTree, Group, Delimiter, Span};
use proc_macro2::{Delimiter, Group, Ident, Literal, Punct, Spacing, Span, TokenStream, TokenTree};
// use std::str::FromStr;

// mod graph_operations;
// mod fbs_graphs;
// mod iter_graphs;

#[proc_macro]
pub fn directory_representation(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as LitStr).value();
    let dir_path = Path::new(&input);
    directory_representation_inner(dir_path).expect("").into()
    // flame::dump_html(&mut std::fs::File::create("flame-graph.html").unwrap()).unwrap();
    // flame::dump_json(&mut std::fs::File::create("flame.json").unwrap()).unwrap();
}

struct File {
    path: PathBuf,

    dir_counts: HashMap<String, usize>,
    stem_word_counts: HashMap<String, usize>,
    ext_counts: HashMap<String, usize>,

    dir_tokens: Vec<(String, usize)>,
    stem_word_tokens: Vec<(String, usize)>,
    ext_tokens: Vec<(String, usize)>,

    entropy: f64,
}

impl File {
    fn from_path(path: PathBuf) -> Result<Self> {
        let mut dir_counts = HashMap::new();
        let mut dir_count = 0usize;
        if let Some(parent) = path.parent() {
            for component in parent.components() {
                let Component::Normal(dir) = component else {
                    continue;
                };
                let word = dir.to_str().ok_or(anyhow!(""))?;
                *dir_counts.entry(word.to_owned()).or_insert(0usize) += 1;
                dir_count += 1;
            }
        }
        let mut stem = path
            .file_name()
            .ok_or(anyhow!(""))?
            .to_str()
            .ok_or(anyhow!(""))?;
        let mut ext_counts = HashMap::new();
        let mut ext_count = 0usize;
        if let Some((new_stem, exts)) = stem.split_once(".") {
            stem = new_stem;
            for etx in exts.split(".") {
                *ext_counts.entry(etx.to_owned()).or_insert(0usize) += 1;
                ext_count += 1;
            }
        }
        let mut stem_word_counts = HashMap::new();
        let mut stem_word_count = 0usize;
        for word in stem.split("_") {
            *stem_word_counts.entry(word.to_owned()).or_insert(0usize) += 1;
            stem_word_count += 1;
        }

        let mut dir_tokens = Vec::with_capacity(dir_count);
        for (word, &count) in &dir_counts {
            for version in 0..count {
                dir_tokens.push((word.to_owned(), version));
            }
        }
        let mut stem_word_tokens = Vec::with_capacity(stem_word_count);
        for (word, &count) in &stem_word_counts {
            for version in 0..count {
                stem_word_tokens.push((word.to_owned(), version));
            }
        }
        let mut ext_tokens = Vec::with_capacity(ext_count);
        for (word, &count) in &ext_counts {
            for version in 0..count {
                ext_tokens.push((word.to_owned(), version));
            }
        }

        Ok(Self {
            path,

            dir_counts,
            stem_word_counts,
            ext_counts,

            dir_tokens,
            stem_word_tokens,
            ext_tokens,

            entropy: 1.0,
        })
    }
    fn path_lit_str(&self) -> Result<LitStr> {
        let value = self
            .path
            .to_str()
            .ok_or(anyhow!("file path to_str failed"))?;
        Ok(LitStr::new(value, Span::call_site()))
    }
    fn update_entropy(
        &mut self,
        dir_token_counts: &HashMap<(String, usize), usize>,
        stem_token_counts: &HashMap<(String, usize), usize>,
        ext_token_counts: &HashMap<(String, usize), usize>,
        file_count: usize,
    ) {
        self.entropy = 1.0;
        for token in &self.dir_tokens {
            self.entropy *= dir_token_counts[token] as f64 / file_count as f64;
        }
        for token in &self.stem_word_tokens {
            self.entropy *= stem_token_counts[token] as f64 / file_count as f64;
        }
        for token in &self.ext_tokens {
            self.entropy *= ext_token_counts[token] as f64 / file_count as f64;
        }
        // dbg!(self.entropy);
    }
}

fn max_counts(counts: impl Iterator<Item = (String, usize)>) -> Vec<(String, usize)> {
    let mut max_counts = HashMap::new();
    for (word, count) in counts {
        max_counts
            .entry(word)
            .and_modify(|v| {
                if count > *v {
                    *v = count;
                }
            })
            .or_insert(count);
    }
    let mut max_counts = max_counts.into_iter().collect::<Vec<_>>();
    max_counts.sort();
    max_counts
}

fn all_tokens(max_counts: &Vec<(String, usize)>) -> Vec<(&String, Option<usize>)> {
    let mut tokens = Vec::new();
    for (word, max_count) in max_counts {
        if *max_count == 1 {
            tokens.push((word, None));
        } else {
            for i in 0..*max_count {
                tokens.push((word, Some(i)));
            }
        }
    }
    tokens
}

fn tokens_associated_files<'a>(
    max_counts: &Vec<(String, usize)>,
    files: &Vec<File>,
    get_count: fn(&File, &String) -> Option<usize>,
) -> Vec<Vec<usize>> {
    let mut tokens_associated_files = Vec::new();
    for (word, max_count) in max_counts {
        let mut current_tokens_associated_files = vec![Vec::new(); *max_count];
        for (file_index, file) in files.iter().enumerate() {
            let Some(count) = get_count(file, word) else {
                continue;
            };
            for i in 0..count {
                current_tokens_associated_files[i].push(file_index);
            }
        }
        tokens_associated_files.extend_from_slice(&current_tokens_associated_files);
    }
    tokens_associated_files
}

fn constant(token: &(&String, Option<usize>), token_associated_files: &Vec<usize>) -> TokenStream {
    #[cfg(debug_assertions)]
    assert!(token_associated_files.is_sorted());

    let constant_name = match token.1 {
        None => format_ident!(
            "_{}",
            token
                .0
                .chars()
                .map(|c| if c.is_alphanumeric() { c } else { '_' })
                .collect::<String>()
        ),
        Some(i) => format_ident!(
            "_{}_{}",
            token
                .0
                .chars()
                .map(|c| if c.is_alphanumeric() { c } else { '_' })
                .collect::<String>(),
            i
        ),
    };
    let array_elements = token_associated_files
        .iter()
        .map(|file_index| TokenTree::Literal(Literal::usize_unsuffixed(*file_index)));

    quote! {
        pub const #constant_name: &[usize] = &[#(#array_elements,)*];
    }
}

fn directory_representation_inner<P: AsRef<Path>>(dir_path: P) -> Result<proc_macro2::TokenStream> {
    let mut files = Vec::new();
    let mut dir_token_counts = HashMap::new();
    let mut stem_token_counts = HashMap::new();
    let mut ext_token_counts = HashMap::new();
    for dir_entry in WalkDir::new(dir_path)
        .into_iter()
        .filter_map(Result::ok)
        .filter(|e| e.file_type().is_file())
    {
        let path = dir_entry.path();
        if path.is_file() {
            let file = File::from_path(path.to_owned())?;
            for token in &file.dir_tokens {
                dir_token_counts
                    .entry(token.to_owned())
                    .and_modify(|count| *count += 1)
                    .or_insert(0usize);
            }
            for token in &file.stem_word_tokens {
                stem_token_counts
                    .entry(token.to_owned())
                    .and_modify(|count| *count += 1)
                    .or_insert(0usize);
            }
            for token in &file.ext_tokens {
                ext_token_counts
                    .entry(token.to_owned())
                    .and_modify(|count| *count += 1)
                    .or_insert(0usize);
            }
            files.push(file);
        }
    }
    let file_count = files.len();
    for file in &mut files {
        file.update_entropy(
            &dir_token_counts,
            &stem_token_counts,
            &ext_token_counts,
            file_count,
        );
    }
    files.sort_by(|a, b| a.path.cmp(&b.path));
    files.sort_by(|a, b| {
        a.entropy
            .partial_cmp(&b.entropy)
            .expect("float ordering failed")
            .reverse()
    });

    let file_paths_len = TokenTree::Literal(Literal::usize_unsuffixed(files.len()));
    let mut file_paths_elements = Vec::new();
    for file in &files {
        file_paths_elements.push(file.path_lit_str()?);
    }

    let dir_max_counts = max_counts(files.iter().flat_map(|file| file.dir_counts.clone()));
    let stem_word_max_counts =
        max_counts(files.iter().flat_map(|file| file.stem_word_counts.clone()));
    let ext_max_counts = max_counts(files.iter().flat_map(|file| file.ext_counts.clone()));

    let all_dir_tokens = all_tokens(&dir_max_counts);
    let all_stem_word_tokens = all_tokens(&stem_word_max_counts);
    let all_ext_tokens = all_tokens(&ext_max_counts);

    let dir_tokens_associated_files =
        tokens_associated_files(&dir_max_counts, &files, |file, word| {
            file.dir_counts.get(word).copied()
        });
    let stem_word_tokens_associated_files =
        tokens_associated_files(&stem_word_max_counts, &files, |file, word| {
            file.stem_word_counts.get(word).copied()
        });
    let ext_tokens_associated_files =
        tokens_associated_files(&ext_max_counts, &files, |file, word| {
            file.ext_counts.get(word).copied()
        });

    let dir_constants = all_dir_tokens
        .iter()
        .zip(&dir_tokens_associated_files)
        .map(|(token, token_associated_files)| constant(token, token_associated_files));
    let stem_word_constants = all_stem_word_tokens
        .iter()
        .zip(&stem_word_tokens_associated_files)
        .map(|(token, token_associated_files)| constant(token, token_associated_files));
    let ext_constants = all_ext_tokens
        .iter()
        .zip(&ext_tokens_associated_files)
        .map(|(token, token_associated_files)| constant(token, token_associated_files));

    Ok(quote! {
        pub const FILE_PATHS: [&'static str; #file_paths_len] = [
            #(#file_paths_elements,)*
        ];
        pub mod dir {
            #(#dir_constants)*
        }
        pub mod stem_word {
            #(#stem_word_constants)*
        }
        pub mod ext {
            #(#ext_constants)*
        }
    })
}

// #[derive(Debug)]
// struct DirectoryTokens {
//     file_token_indices_original: Vec<Vec<usize>>,
//     tokens: Vec<(String, usize)>,
//     inverse_tokens_sort: Vec<usize>,
//     color_bounds: Vec<usize>,
//     color_count: usize,
//     possible_files_bounds: Vec<usize>,
//     total_possible_files: usize,
//     grouped_tokens: Vec<Vec<usize>>,
// }

// impl DirectoryTokens {
//     pub fn from_file_paths<P: AsRef<Path>>(file_paths: &Vec<P>) -> Result<Self> {
//         // tokenise
//         let mut file_word_counts = Vec::with_capacity(file_paths.len());
//         for file_path in file_paths {
//             let mut word_counts = HashMap::new();
//             for token in file_path
//                 .as_ref()
//                 .file_stem()
//                 .ok_or(anyhow!("Could not get file stem"))?
//                 .to_str()
//                 .ok_or(anyhow!("Could not convert file stem to str"))?
//                 .split("_")
//             {
//                 *word_counts.entry(token.to_owned()).or_insert(0) += 1;
//             }
//             file_word_counts.push(word_counts);
//         }
//         // get max counts
//         let mut max_word_counts = HashMap::new();
//         for counts in &file_word_counts {
//             for (word, &count) in counts {
//                 max_word_counts
//                     .entry(word.clone())
//                     .and_modify(|v| {
//                         if count > *v {
//                             *v = count;
//                         }
//                     })
//                     .or_insert(count);
//             }
//         }
//         let mut max_word_counts = max_word_counts.into_iter().collect::<Vec<_>>();
//         max_word_counts.sort();
//         // [(word, index), (word, index), ...]
//         let tokens = max_word_counts
//             .into_iter()
//             .map(|(word, max_count)| (0..max_count).map(move |index| (word.clone(), index)))
//             .flatten()
//             .collect::<Vec<_>>();
//         // token_to_index[(word, index)] = token_index
//         let mut token_to_index = HashMap::new();
//         for (i, token) in tokens.iter().enumerate() {
//             token_to_index.insert(token.clone(), i);
//         }
//         // file_paths[i] <-> file_tokens[i] = FixedBitSet with contained token indices set
//         let mut file_token_indices_original = Vec::with_capacity(file_word_counts.len());
//         for counts in file_word_counts {
//             let token_to_index = &token_to_index;
//             let mut token_indices = Vec::new();
//             for (word, &count) in &counts {
//                 token_indices
//                     .extend((0..count).map(move |index| token_to_index[&(word.clone(), index)]));
//             }
//             file_token_indices_original.push(token_indices);
//         }
//         // undirected graph where there is an edge between tokens if they ever appear in the same file
//         let mut token_graph = vec![IndexSet::new(); tokens.len()];
//         for token_index in 0..tokens.len() {
//             for file_token_indices in &file_token_indices_original {
//                 if file_token_indices.contains(&token_index) {
//                     token_graph[token_index].extend(file_token_indices);
//                 }
//             }
//         }
//         // color token graph to find sets of mutually exclusive tokens
//         let (color_count, coloring) = iter_graphs::graph_coloring(&token_graph, tokens.len());
//         let mut num_of_colors = vec![0usize; color_count];
//         for &color in &coloring {
//             num_of_colors[color] += 1;
//         }
//         let mut tokens_sort = (0..tokens.len()).collect::<Vec<_>>();
//         tokens_sort.sort_by(|&token_index_0, &token_index_1| {
//             num_of_colors[coloring[token_index_0]].cmp(&num_of_colors[coloring[token_index_1]])
//         });
//         let mut inverse_tokens_sort = vec![0; tokens_sort.len()];
//         for (i, &j) in tokens_sort.iter().enumerate() {
//             inverse_tokens_sort[j] = i;
//         }
//         num_of_colors.sort();
//         let mut color_bounds = Vec::with_capacity(color_count + 1);
//         let mut bound = 0;
//         color_bounds.push(bound);
//         for &num_of_color in &num_of_colors {
//             bound += num_of_color;
//             color_bounds.push(bound);
//         }
//         let mut possible_files_bounds = Vec::with_capacity(color_count + 1);
//         let mut bound = 1;
//         possible_files_bounds.push(bound);
//         for &num_of_color in &num_of_colors {
//             bound *= num_of_color + 1;
//             possible_files_bounds.push(bound);
//         }
//         let total_possible_files = bound;
//         let mut grouped_tokens = vec![Vec::new(); color_count];
//         for color in 0..color_count {
//             for token_index in color_bounds[color]..color_bounds[color + 1] {
//                 grouped_tokens[color].push(tokens_sort[token_index]);
//             }
//         }
//         Ok(Self {
//             tokens,
//             inverse_tokens_sort,
//             color_bounds,
//             file_token_indices_original,
//             color_count,
//             possible_files_bounds,
//             total_possible_files,
//             grouped_tokens,
//         })
//     }
//     fn token_indices_of_color(&self, color: usize) -> Range<usize> {
//         self.color_bounds[color]..self.color_bounds[color + 1]
//     }
//     fn file_token_indices(&self) -> impl Iterator<Item = Vec<Option<usize>>> {
//         self.file_token_indices_original
//             .iter()
//             .map(|file_token_indices| {
//                 let mut token_indices = file_token_indices
//                     .iter()
//                     .map(|&index| self.inverse_tokens_sort[index])
//                     .collect::<Vec<_>>();
//                 token_indices.sort();
//                 let mut v = vec![None; self.color_count];
//                 let mut i = 0;
//                 let mut color = 0;
//                 while i < token_indices.len() && color < self.color_count {
//                     if self
//                         .token_indices_of_color(color)
//                         .contains(&token_indices[i])
//                     {
//                         v[color] = Some(token_indices[i]);
//                         i += 1;
//                     }
//                     color += 1;
//                 }
//                 v
//             })
//     }
//     fn possible_file_edges(&self, index: usize) -> impl Iterator<Item = usize> {
//         (0..self.color_count).filter_map(move |color| {
//             let x =
//                 (index % self.possible_files_bounds[color + 1]) / self.possible_files_bounds[color];
//             if x != 0 {
//                 Some(index - x * self.possible_files_bounds[color])
//             } else {
//                 None
//             }
//         })
//     }
//     fn real_files_as_possible_file_indices(&self) -> impl Iterator<Item = usize> {
//         self.file_token_indices().map(|x| {
//             let mut index = 0;
//             for color in 0..self.color_count {
//                 if let Some(x) = x[color] {
//                     index += (x - self.color_bounds[color] + 1) * self.possible_files_bounds[color];
//                 }
//             }
//             index
//         })
//     }
//     fn file_index(&self, token_indices: Vec<Option<usize>>) -> usize {
//         let mut result = 0;
//         for (color, token_index) in token_indices.iter().enumerate() {
//             if let Some(token_index) = token_index {
//                 result += self.possible_files_bounds[color]
//                     * (token_index - self.color_bounds[color] + 1);
//             }
//         }
//         result
//     }
// }

// #[test]
// fn directory_tokens_test() -> Result<()> {
//     let mut file_paths = Vec::new();
//     for dir_entry in std::fs::read_dir(
//         &"../assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default",
//     )? {
//         let path = dir_entry?.path();
//         if path.is_file() {
//             file_paths.push(path);
//         }
//     }
//     // sort file paths as the order may affect predictions made
//     file_paths.sort();
//     let d = DirectoryTokens::from_file_paths(&file_paths)?;
//     println!("{:?}", d);
//     for i in d.file_tokens() {
//         println!("{:?}", i.collect::<Vec<_>>());
//     }
//     println!("{:?}", d.file_token_indices().collect::<Vec<_>>());
//     for n in 0..d.color_count {
//         println!(
//             "{:?}",
//             d.token_indices_of_color(n)
//                 .map(|x| d.get_token(x))
//                 .collect::<Vec<_>>()
//         );
//     }
//     for n in 0..100 {
//         println!("{:?}", d.possible_file(n));
//     }
//     println!("{:?}", d.possible_files_bounds);
//     for n in 0..100 {
//         dbg!(n);
//         println!("node = {:?}", d.possible_file(n));
//         for edge in d.possible_file_edges(n) {
//             dbg!(edge);
//             println!("edge = {:?}", d.possible_file(edge));
//         }
//     }
//     for x in d.real_files_as_possible_file_indices() {
//         let x = d.possible_file(x);
//         println!(
//             "{:?}",
//             x.iter()
//                 .map(|x| match x {
//                     Some(x) => &d.get_token(*x).0,
//                     None => "_",
//                 })
//                 .collect::<Vec<_>>()
//         );
//     }

//     Ok(())
// }

// struct DirectoryRepresentationIntermediary<'a> {
//     dir_ident: proc_macro2::Ident,
//     path: syn::LitStr,
//     file_paths: Vec<PathBuf>,

//     dir_paths: Vec<PathBuf>,

//     directory_tokens: DirectoryTokens,

//     predictions: Vec<(usize, usize)>,

//     ignore: &'a str,
// }

// impl<'a> DirectoryRepresentationIntermediary<'a> {
//     fn from_path<P: AsRef<Path>>(dir: P, ignore: &'a str) -> Result<Self> {
//         // it's fine if non-utf8 characters get replaced
//         let dir_ident = dir_to_ident(
//             &dir.as_ref()
//                 .file_name()
//                 .ok_or(anyhow!("Could not get file_name"))?
//                 .to_string_lossy(),
//         );
//         // it's not fine if non-utf8 characters get replaced
//         let path = syn::LitStr::new(
//             dir.as_ref()
//                 .to_str()
//                 .ok_or(anyhow!("Could not convert file name to str"))?,
//             proc_macro2::Span::call_site(),
//         );
//         // get all the contained files
//         let mut file_paths = Vec::new();
//         let mut dir_paths = Vec::new();
//         for dir_entry in std::fs::read_dir(&dir)? {
//             let path = dir_entry?.path();
//             if path.is_file() {
//                 file_paths.push(
//                     path.strip_prefix(ignore)
//                         .expect("path does not have prefix ignore")
//                         .to_owned(),
//                 );
//             } else if path.is_dir() {
//                 dir_paths.push(path);
//             }
//         }
//         // sort file paths as the order may affect predictions made
//         file_paths.sort();
//         let directory_tokens = DirectoryTokens::from_file_paths(&file_paths)?;

//         // create edges from all possible files to possible files where one token has been added
//         let mut graph = vec![Vec::new(); directory_tokens.total_possible_files];
//         let mut transposed_graph = vec![Vec::new(); directory_tokens.total_possible_files];
//         for i in 0..directory_tokens.total_possible_files {
//             for e in directory_tokens.possible_file_edges(i) {
//                 graph[i].push(e);
//                 transposed_graph[e].push(i);
//             }
//         }
//         // create that graph
//         // edges that represent a low information token being added should appear first
//         // create predictions for what the user might mean that remove certain tokens
//         // by flood filling from real files to possible files with extra tokens

//         let mut initial_predictions = Vec::new();
//         let mut visited = FixedBitSet::with_capacity(directory_tokens.total_possible_files);
//         for index in directory_tokens.real_files_as_possible_file_indices() {
//             initial_predictions.push((index, index));
//             visited.insert(index);
//         }
//         // flood fill graph of possible files to create predictions
//         let mut i = 0;
//         let mut predictions = initial_predictions.clone();
//         while i < predictions.len() {
//             for &edge in transposed_graph[predictions[i].0].iter() {
//                 if visited.contains(edge) {
//                     continue;
//                 }
//                 visited.insert(edge);
//                 predictions.push((edge, predictions[i].1));
//             }
//             i += 1;
//         }
//         predictions.extend(initial_predictions.clone().into_iter());
//         let j = i;
//         // removing tokens to create addition predictions
//         while i < predictions.len() {
//             for &edge in graph[predictions[i].0].iter() {
//                 // doing visited check here is more performant than before the loop
//                 // this also means only the predicitons we want will be added to predictions
//                 if visited.contains(edge) {
//                     continue;
//                 }
//                 visited.insert(edge);
//                 predictions.push((edge, predictions[i].1));
//             }
//             i += 1;
//         }
//         let mut i = j;
//         while i < predictions.len() {
//             for &edge in transposed_graph[predictions[i].0].iter() {
//                 if visited.contains(edge) {
//                     continue;
//                 }
//                 visited.insert(edge);
//                 predictions.push((edge, predictions[i].1));
//             }
//             i += 1;
//         }
//         Ok(Self {
//             dir_ident,
//             path,
//             file_paths,
//             dir_paths,
//             directory_tokens,
//             predictions,
//             ignore,
//         })
//     }

//     fn to_token_stream(&self) -> Result<proc_macro2::TokenStream> {
//         let mx_enums = self.directory_tokens.grouped_tokens.iter().enumerate().map(
//             |(color, token_indices)| {
//                 let enum_name = format_ident!("_MX_{}", color);
//                 let variants = token_indices.iter().enumerate().map(|(i, &token_index)| {
//                     let ident = token_to_ident(&self.directory_tokens.tokens[token_index]);
//                     let repr = TokenTree::Literal(Literal::usize_unsuffixed(i + 1));
//                     quote! { #ident = #repr }
//                 });
//                 quote! {
//                     #[repr(usize)]
//                     pub enum #enum_name {
//                         None = 0,
//                         #(#variants,)*
//                     }
//                 }
//             },
//         );
//         let mut file_function_arms = Vec::new();
//         for (file_index, file_path) in self
//             .directory_tokens
//             .file_token_indices()
//             .map(|token_indices| {
//                 // dbg!(&token_indices);
//                 self.directory_tokens.file_index(token_indices)
//             })
//             .zip(&self.file_paths)
//         {
//             let file_index = TokenTree::Literal(Literal::usize_unsuffixed(file_index));
//             let file_path = syn::LitStr::new(
//                 file_path
//                     .to_str()
//                     .ok_or(anyhow!("Could not convert file path to str"))?,
//                 proc_macro2::Span::call_site(),
//             );
//             file_function_arms.push(quote::quote! { #file_index => Some( #file_path ) });
//         }
//         let file_function = quote::quote! {
//             pub fn file(possible_file: usize) -> Option<&'static str> {
//                 match possible_file {
//                     #(#file_function_arms,)*
//                     _ => None
//                 }
//             }
//         };
//         let mut possible_actual_files = self.predictions.clone();
//         possible_actual_files.sort_by_key(|x| x.0);
//         possible_actual_files.dedup_by_key(|x| x.0);
//         if possible_actual_files.len() != 0 {
//             assert_eq!(
//                 possible_actual_files.len(),
//                 self.directory_tokens.total_possible_files
//             );
//         }
//         // dbg!(&possible_actual_files);
//         let predict_function_match =
//             TokenStream::from_iter(possible_actual_files.iter().flat_map(|(possible, actual)| {
//                 [
//                     TokenTree::Literal(Literal::usize_unsuffixed(*actual)),
//                     TokenTree::Punct(Punct::new(',', Spacing::Alone)),
//                 ]
//             }));
//         let num_predictions =
//             TokenTree::Literal(Literal::usize_unsuffixed(possible_actual_files.len()));
//         let predict_function = quote::quote! {
//             pub const PREDICTIONS: [usize; #num_predictions] = [#predict_function_match];
//         };

//         let calculate_input_type =
//             (0..self.directory_tokens.color_count).map(|i| quote::format_ident!("_MX_{}", i));
//         let mut calculate_terms = Vec::new();
//         for color in 0..self.directory_tokens.color_count {
//             let i = Index::from(color);
//             let bound_lit = TokenTree::Literal(Literal::usize_unsuffixed(
//                 self.directory_tokens.possible_files_bounds[color],
//             ));
//             calculate_terms.push(quote! {
//                 + possible_file.#i as usize * #bound_lit
//             });
//         }
//         let calculate_function = quote! {
//             #[inline]
//             pub fn calculate(possible_file: (#(#calculate_input_type,)*)) -> usize {
//                 0 #(#calculate_terms)*
//             }
//         };
//         let mut submodules = Vec::new();
//         for sub_dir in &self.dir_paths {
//             submodules.push(
//                 DirectoryRepresentationIntermediary::from_path(sub_dir, self.ignore)?
//                     .to_token_stream()?,
//             );
//         }

//         let dir_ident = &self.dir_ident;
//         let path = &self.path;
//         Ok(quote::quote! {
//             pub mod #dir_ident {
//                 pub const PATH: &'static str = #path;
//                 #(#mx_enums)*
//                 #file_function
//                 #predict_function
//                 #calculate_function
//                 #(#submodules)*
//             }
//         })
//     }
// }

// fn dir_to_ident(name: &str) -> proc_macro2::Ident {
//     let base = name
//         .chars()
//         .map(|c| if c.is_alphanumeric() { c } else { '_' })
//         .collect::<String>();
//     let base = format!("_{}", base);
//     proc_macro2::Ident::new(&base, Span::call_site())
// }

// fn token_to_ident(token: &(String, usize)) -> proc_macro2::Ident {
//     let base = token
//         .0
//         .chars()
//         .map(|c| if c.is_alphanumeric() { c } else { '_' })
//         .collect::<String>();
//     // two tokens that have the same string should never be in the same enum as they will always be in the same file name
//     let base = format!("_{}", base);
//     proc_macro2::Ident::new(&base, Span::call_site())
// }

// // #[test]
// // fn count_colors() -> Result<()> {
// //     // let s = "../assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Double";
// //     let s = "../assets/bevy_input_prompts/xelu/Xelu_Free_Controller&Key_Prompts/Xbox Series";
// //     let x = DirectoryRepresentationIntermediary::from_path(s, |non_exclusive, num_tokens| {
// //         let order = graph_operations::degeneracy_ordering(&non_exclusive, num_tokens);
// //         graph_operations::greedy_coloring(&non_exclusive, &order, num_tokens)
// //     })?;
// //     println!("degeneracy ordering, greedy: {}", x.mx_count);
// //     let x = DirectoryRepresentationIntermediary::from_path(s, |non_exclusive, num_tokens| {
// //         graph_operations::dsatur(&non_exclusive, num_tokens)
// //     })?;
// //     println!("dsatur: {}", x.mx_count);
// //     let x = DirectoryRepresentationIntermediary::from_path(s, |non_exclusive, num_tokens| {
// //         graph_operations::color_graph(&non_exclusive, num_tokens)
// //     })?;
// //     println!("exact: {}", x.mx_count);
// //     Ok(())
// // }
