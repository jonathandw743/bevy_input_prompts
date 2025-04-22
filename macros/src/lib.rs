use anyhow::{Result, anyhow};
use fixedbitset::FixedBitSet;
use hashbrown::HashMap;
use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use std::{
    borrow::Cow,
    collections::{HashSet, VecDeque, vec_deque},
    iter::once,
    path::{Path, PathBuf},
    rc::Rc,
};
use syn::{Ident, LitStr, parse_macro_input};

mod graph_operations;

#[proc_macro]
pub fn directory_representation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr).value();
    let dir_path = Path::new(&input);
    DirectoryRepresentationIntermediary::from_path(
        dir_path,
        |non_exclusive, num_tokens| {
            let order = graph_operations::degeneracy_ordering(&non_exclusive, num_tokens);
            graph_operations::greedy_coloring(&non_exclusive, &order, num_tokens)
        }, // |non_exclusive, num_tokens| {
           //     graph_operations::cliques(&non_exclusive, num_tokens)
           // let order = graph_operations::degeneracy_ordering(&non_exclusive, num_tokens);
           // graph_operations::greedy_coloring(&non_exclusive, &order, num_tokens)
           // },
    )
    .expect("Could not create directory representation module")
    .to_token_stream()
    .expect("Could not create directory representation module")
    .into()
}

struct DirectoryRepresentationIntermediary {
    dir_ident: Ident,
    path: LitStr,
    file_paths: Vec<PathBuf>,
    dir_paths: Vec<PathBuf>,
    tokens: Vec<(String, usize)>,
    file_tokens: Vec<FixedBitSet>,
    coloring: Vec<usize>,
    color_count: usize,
    color_to_tokens: Vec<FixedBitSet>,
    graph_coloring: fn(&Vec<FixedBitSet>, usize) -> (usize, Vec<usize>),
    possible_files: VecDeque<Vec<Option<usize>>>,
    predictions: Vec<(usize, usize)>,
}

impl DirectoryRepresentationIntermediary {
    fn from_path<P: AsRef<Path>>(
        dir: P,
        graph_coloring: fn(&Vec<FixedBitSet>, usize) -> (usize, Vec<usize>),
    ) -> Result<Self> {
        flame::start("all");
        flame::start("setup");
        // it's fine if non-utf8 characters get replaced
        let dir_ident = dir_to_ident(
            &dir.as_ref()
                .file_name()
                .ok_or(anyhow!("Could not get file_name"))?
                .to_string_lossy(),
        );
        // it's not fine if non-utf8 characters get replaced
        let path = syn::LitStr::new(
            dir.as_ref()
                .to_str()
                .ok_or(anyhow!("Could not convert file name to str"))?,
            proc_macro2::Span::call_site(),
        );
        // get all the contained files
        let mut file_paths = Vec::new();
        let mut dir_paths = Vec::new();
        for dir_entry in std::fs::read_dir(&dir)? {
            let path = dir_entry?.path();
            if path.is_file() {
                file_paths.push(path);
            } else if path.is_dir() {
                dir_paths.push(path);
            }
        }
        // sort file paths as the order may affect predictions made
        file_paths.sort();
        // tokenise
        let mut file_word_counts = Vec::new();
        for file_path in &file_paths {
            let mut word_counts = HashMap::new();
            for token in file_path
                .file_stem()
                .ok_or(anyhow!("Could not get file stem"))?
                .to_str()
                .ok_or(anyhow!("Could not convert file stem to str"))?
                .split("_")
            {
                *word_counts.entry(token.to_owned()).or_insert(0) += 1;
            }
            file_word_counts.push(word_counts);
        }
        // get max counts
        let mut max_word_counts = HashMap::new();
        for counts in &file_word_counts {
            for (word, &count) in counts {
                max_word_counts
                    .entry(word.clone())
                    .and_modify(|v| {
                        if count > *v {
                            *v = count;
                        }
                    })
                    .or_insert(count);
            }
        }
        let mut max_word_counts = max_word_counts.into_iter().collect::<Vec<_>>();
        max_word_counts.sort();
        // [(word, index), (word, index), ...]
        let tokens = max_word_counts
            .into_iter()
            .map(|(word, max_count)| (0..max_count).map(move |index| (word.clone(), index)))
            .flatten()
            .collect::<Vec<_>>();
        // token_to_index[(word, index)] = token_index
        let mut token_to_index = HashMap::new();
        for (i, token) in tokens.iter().enumerate() {
            token_to_index.insert(token.clone(), i);
        }
        // file_paths[i] <-> file_tokens[i] = FixedBitSet with contained token indices set
        let mut file_tokens = Vec::with_capacity(file_word_counts.len());
        for counts in file_word_counts {
            let token_to_index = &token_to_index;
            let mut token_indices = Vec::new();
            for (word, &count) in &counts {
                token_indices
                    .extend((0..count).map(move |index| token_to_index[&(word.clone(), index)]));
            }
            file_tokens.push(FixedBitSet::from_iter(token_indices));
        }
        // undirected graph where there is an edge between tokens if they ever appear in the same file
        let mut token_graph = vec![FixedBitSet::with_capacity(tokens.len()); tokens.len()];
        for token_index in 0..tokens.len() {
            for file_tokens in &file_tokens {
                if file_tokens.contains(token_index) {
                    token_graph[token_index].union_with(file_tokens);
                }
            }
        }
        flame::end("setup");
        flame::start("graph coloring");
        // color token graph to find sets of mutually exclusive tokens
        let (color_count, coloring) = graph_coloring(&token_graph, tokens.len());
        flame::end("graph coloring");

        let mut num_of_color = vec![0; color_count];
        for &color in &coloring {
            num_of_color[color] += 1;
        }
        let mut colors_sort = (0..color_count).collect::<Vec<_>>();
        // negative allows low information tokens to be set first later
        colors_sort.sort_by_key(|&color| -num_of_color[color]);
        let mut inverse_colors_sort = vec![0; color_count];
        for color in 0..color_count {
            inverse_colors_sort[colors_sort[color]] = color;
        }
        // token_index -> color
        let coloring = coloring
            .into_iter()
            .map(|x| inverse_colors_sort[x])
            .collect::<Vec<_>>();
        // color -> [token_index, token_index, ...]
        let mut color_to_tokens = vec![FixedBitSet::with_capacity(tokens.len()); color_count];
        for (token_index, &color) in coloring.iter().enumerate() {
            color_to_tokens[color].insert(token_index);
        }

        // #[cfg(debug_assertions)]
        // {
        //     let min_colors = file_tokens
        //         .iter()
        //         .map(|file_tokens| file_tokens.count_ones(..))
        //         .max()
        //         .unwrap_or(0);
        //     dbg!(
        //         min_colors == color_count,
        //         dir.as_ref(),
        //         min_colors,
        //         color_count
        //     );
        //     let mut num_possible_files = 1;
        //     for c in &color_to_tokens {
        //         num_possible_files *= c.count_ones(..) + 1;
        //     }
        //     dbg!(num_possible_files);
        //     dbg!("-----");
        // }
        flame::start("creating possible files");
        // create all possible files
        let mut possible_files = VecDeque::new();
        possible_files.push_back(Vec::with_capacity(color_count));
        while let Some(partial) = possible_files.pop_front() {
            if partial.len() == color_count {
                possible_files.push_front(partial);
                break;
            }
            {
                let mut new_partial = partial.clone();
                new_partial.push(None);
                possible_files.push_back(new_partial);
            }
            for token_index in color_to_tokens[partial.len()].ones() {
                let mut new_partial = partial.clone();
                new_partial.push(Some(token_index));
                possible_files.push_back(new_partial);
            }
        }
        flame::end("creating possible files");
        flame::start("creating graph");
        // create edges from all possible files to possible files where one token has been added
        flame::start("allocation");
        let mut graph = vec![Vec::with_capacity(possible_files.len()); possible_files.len()];
        let mut transposed_graph = vec![Vec::with_capacity(possible_files.len()); possible_files.len()];
        flame::end("allocation");
        flame::start("n");
        let mut ns = Vec::with_capacity(color_count + 1);
        let mut n = 1;
        ns.push(n);
        for c in &color_to_tokens {
            n *= c.count_ones(..) + 1;
            ns.push(n);
        }
        flame::end("n");
        flame::start("edges");
        for i in 0..n {
            for color in (0..color_count).rev() {
                let count = color_to_tokens[color].count_ones(..);
                let offset = i % ns[color + 1] - i % ns[color];
                if offset != 0 {
                    graph[i].push(i - offset);
                    transposed_graph[i - offset].push(i);
                }
            }
        }
        flame::end("edges");
        // dbg!(graph.iter().map(|x| format!("{}", x)).collect::<Vec<_>>());
        flame::end("creating graph");

        //     for j in (0..n).step_by(m * (count + 1)) {
        //         for k in (0..(m * (count + 1))).step_by(m) {
        //             // no edge to self
        //             if k == 0 {
        //                 continue;
        //             }
        //             edges.push((j, j + k));
        //         }
        //     }
        // }

        // create that graph
        // edges that represent a low information token being added should appear first
        flame::start("graph setup");
        // create predictions for what the user might mean that remove certain tokens
        // by flood filling from real files to possible files with extra tokens
        let mut token_to_index_in_colors = vec![0; tokens.len()];
        for (color, tokens) in color_to_tokens.iter().enumerate() {
            for (i, token) in tokens.ones().enumerate() {
                token_to_index_in_colors[token] = i;
            }
        }
        let mut initial_predictions = Vec::new();
        let mut visited = FixedBitSet::with_capacity(possible_files.len());
        for file_tokens in &file_tokens {
            let mut file_tokens_possible_file = vec![None; color_count];
            for file_token in file_tokens.ones() {
                file_tokens_possible_file[coloring[file_token]] = Some(file_token);
            }
            let mut m = 1;
            let mut index = 0;
            for color in (0..color_count).rev() {
                let count = color_to_tokens[color].count_ones(..);
                if let Some(file_token) = file_tokens_possible_file[color] {
                    index += m * (token_to_index_in_colors[file_token] + 1);
                }
                m *= count + 1;
            }
            // NOTE: if files aren't deterministically ordered, predictions could change
            // NOTE: just sorting by file path for now (above)
            initial_predictions.push((index, index));
            visited.insert(index);
        }
        flame::end("graph setup");
        flame::start("traversing graph");
        // flood fill graph of possible files to create predictions
        flame::start("1");
        let mut i = 0;
        let mut predictions = initial_predictions.clone();
        // removing tokens to create addition predictions
        while i < predictions.len() {
            for &edge in graph[predictions[i].0].iter() {
                // doing visited check here is more performant than before the loop
                // this also means only the predicitons we want will be added to predictions
                if visited.contains(edge) {
                    continue;
                }
                visited.insert(edge);
                predictions.push((edge, predictions[i].1));
            }
            i += 1;
        }
        flame::end("1");
        flame::start("2");

        predictions.extend(initial_predictions.clone().into_iter());
        // adding tokens to create removal predictions
        while i < predictions.len() {
            for &edge in transposed_graph[predictions[i].0].iter() {
                if visited.contains(edge) {
                    continue;
                }
                visited.insert(edge);
                predictions.push((edge, predictions[i].1));
            }
            i += 1;
        }
        flame::end("2");
        flame::start("3");
        let mut i = 0;
        // removing tokens to create all other predictions
        while i < predictions.len() {
            for &edge in transposed_graph[predictions[i].0].iter() {
                if visited.contains(edge) {
                    continue;
                }
                visited.insert(edge);
                predictions.push((edge, predictions[i].1));
            }
            i += 1;
        }
        flame::end("3");
        flame::end("traversing graph");

        // dbg!(format!("{}", visited.union(&transposed_visited).collect::<FixedBitSet>()));
        // dbg!(format!("{}", visited));

        // let predictions = [predictions, transposed_predictions].concat();
        // let predictions = predictions.ext

        // let d = token_to_index[&("down".to_string(), 0)];
        // let a = token_to_index[&("arrow".to_string(), 0)];
        // let o = token_to_index[&("outline".to_string(), 0)];
        // // dbg!(d);
        // for (i, possible_file) in possible_files.iter().enumerate() {
        //     if possible_file == &vec![Some(d), Some(a), None, Some(o), None] {
        //         dbg!(i);
        //         dbg!(&possible_files[i]);
        //         dbg!(&graph[i].ones().collect::<Vec<_>>());
        //         for edge in graph[i].ones() {
        //             dbg!(&possible_files[edge]);
        //         }
        //         dbg!(&transposed_graph[i].ones().collect::<Vec<_>>());
        //         for edge in transposed_graph[i].ones() {
        //             dbg!(&possible_files[edge]);
        //         }
        //     }
        // }
        flame::end("all");
        flame::dump_html(&mut std::fs::File::create("flame-graph.html").unwrap()).unwrap();
        flame::dump_json(&mut std::fs::File::create("flame.json").unwrap()).unwrap();

        Ok(Self {
            dir_ident,
            path,
            file_paths,
            dir_paths,
            tokens,
            file_tokens,
            coloring,
            color_count,
            color_to_tokens,
            graph_coloring,

            possible_files,
            predictions,
        })
    }

    fn to_token_stream(&self) -> Result<proc_macro2::TokenStream> {
        // create mutually exculsive enums using graph coloring
        let mx_enums = self
            .color_to_tokens
            .iter()
            .enumerate()
            .map(|(color, token_indices)| {
                let enum_name = format_ident!("_MX_{}", color);
                let variants = token_indices
                    .ones()
                    .map(|token_index| token_to_ident(&self.tokens[token_index]));
                quote! {
                    #[derive(Debug)]
                    pub enum #enum_name {
                        #(#variants,)*
                    }
                }
            });
        // create function from possible files to paths
        let mut file_function_arms = Vec::new();
        for (file_tokens, file_path) in self.file_tokens.iter().zip(&self.file_paths) {
            let mut variants = vec![quote! { None }; self.color_count];
            for token_index in file_tokens.ones() {
                let color = self.coloring[token_index];
                let enum_name = format_ident!("_MX_{}", color);
                let variant = token_to_ident(&self.tokens[token_index]);
                variants[color] = quote! { Some( #enum_name :: #variant ) };
            }
            let file_path = LitStr::new(
                file_path
                    .to_str()
                    .ok_or(anyhow!("Could not convert file path to str"))?,
                Span::call_site(),
            );
            file_function_arms.push(quote! { (#(#variants,)*) => Some( #file_path ) });
        }
        let function_input_type = (0..self.color_count).map(|i| {
            let enum_name = format_ident!("_MX_{}", i);
            quote! {
                Option< #enum_name >
            }
        });
        let file_function_input_type = function_input_type.clone();
        let file_function = quote! {
            pub fn file(possible_file: (#(#file_function_input_type,)*)) -> Option<&'static str> {
                match possible_file {
                    #(#file_function_arms,)*
                    _ => None
                }
            }
        };
        // create function that maps from possible files to actual files by removing tokens
        let mut predict_function_arms = Vec::new();
        for (possible, actual) in &self.predictions {
            let possible_variants =
                self.possible_files[*possible]
                    .iter()
                    .enumerate()
                    .map(|(i, token_index)| match token_index {
                        Some(token_index) => {
                            let enum_name = format_ident!("_MX_{}", i);
                            let variant = token_to_ident(&self.tokens[*token_index]);
                            quote! { Some( #enum_name :: #variant ) }
                        }
                        None => quote! { None },
                    });
            let actual_variants =
                self.possible_files[*actual]
                    .iter()
                    .enumerate()
                    .map(|(i, token_index)| match token_index {
                        Some(token_index) => {
                            let enum_name = format_ident!("_MX_{}", i);
                            let variant = token_to_ident(&self.tokens[*token_index]);
                            quote! { Some( #enum_name :: #variant ) }
                        }
                        None => quote! { None },
                    });
            predict_function_arms.push(quote! {
                (#(#possible_variants,)*) => (#(#actual_variants,)*)
            });
        }
        let predict_function_input_type = function_input_type.clone();
        let predict_function = quote! {
            pub fn predict(possible_file: (#(#predict_function_input_type,)*)) -> (#(#function_input_type,)*) {
                match possible_file {
                    #(#predict_function_arms,)*
                }
            }
        };
        // process sub directories
        let mut submodules = Vec::new();
        for sub_dir in &self.dir_paths {
            submodules.push(
                DirectoryRepresentationIntermediary::from_path(sub_dir, self.graph_coloring)?
                    .to_token_stream()?,
            );
        }

        let dir_ident = &self.dir_ident;
        let path = &self.path;

        Ok(quote! {
            pub mod #dir_ident {
                pub const PATH: &'static str = #path;
                #(#mx_enums)*
                #file_function
                #predict_function
                #(#submodules)*
            }
        })
    }
}

fn dir_to_ident(name: &str) -> proc_macro2::Ident {
    let base = name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>();
    let base = format!("_{}", base);
    syn::parse_str::<proc_macro2::Ident>(&base).unwrap()
}

fn token_to_ident(token: &(String, usize)) -> proc_macro2::Ident {
    let base = token
        .0
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>();
    // two tokens that have the same string should never be in the same enum as they will always be in the same file name
    let base = format!("_{}", base);
    syn::parse_str(&base).expect("Could not parse str")
}

#[test]
fn count_colors() -> Result<()> {
    // let s = "../assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Double";
    let s = "../assets/bevy_input_prompts/xelu/Xelu_Free_Controller&Key_Prompts/Xbox Series";
    let x = DirectoryRepresentationIntermediary::from_path(s, |non_exclusive, num_tokens| {
        let order = graph_operations::degeneracy_ordering(&non_exclusive, num_tokens);
        graph_operations::greedy_coloring(&non_exclusive, &order, num_tokens)
    })?;
    println!("degeneracy ordering, greedy: {}", x.mx_count);
    let x = DirectoryRepresentationIntermediary::from_path(s, |non_exclusive, num_tokens| {
        graph_operations::dsatur(&non_exclusive, num_tokens)
    })?;
    println!("dsatur: {}", x.mx_count);
    let x = DirectoryRepresentationIntermediary::from_path(s, |non_exclusive, num_tokens| {
        graph_operations::color_graph(&non_exclusive, num_tokens)
    })?;
    println!("exact: {}", x.mx_count);
    Ok(())
}
