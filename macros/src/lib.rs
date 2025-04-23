use anyhow::{Result, anyhow};
use fixedbitset::FixedBitSet;
use hashbrown::HashMap;
use indexmap::IndexSet;
use itertools::Itertools;
use proc_macro2::Span;
use syn::Token;
// use proc_macro2::Span;
// use quote::{format_ident, quote};
use std::{
    borrow::Cow,
    collections::{BTreeSet, HashSet, VecDeque, vec_deque},
    fmt::Debug,
    iter::once,
    ops::Range,
    path::{Path, PathBuf},
    rc::Rc,
};
// use std::str::FromStr;

// mod graph_operations;
mod fbs_graphs;
mod iter_graphs;

#[proc_macro]
pub fn directory_representation(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = syn::parse_macro_input!(input as syn::LitStr).value();
    let dir_path = Path::new(&input);
    let x = DirectoryRepresentationIntermediary::from_path(
        dir_path,
        // |non_exclusive, num_tokens| {
        //     let order = graph_operations::degeneracy_ordering(&non_exclusive, num_tokens);
        //     graph_operations::greedy_coloring(&non_exclusive, &order, num_tokens)
        // }, // |non_exclusive, num_tokens| {
        //     graph_operations::cliques(&non_exclusive, num_tokens)
        // let order = graph_operations::degeneracy_ordering(&non_exclusive, num_tokens);
        // graph_operations::greedy_coloring(&non_exclusive, &order, num_tokens)
        // },
    )
    .expect("Could not create directory representation module")
    .to_token_stream()
    .expect("Could not create directory representation module")
    .into();
    flame::dump_html(&mut std::fs::File::create("flame-graph.html").unwrap()).unwrap();
    flame::dump_json(&mut std::fs::File::create("flame.json").unwrap()).unwrap();
    x
}

// struct Graph(Vec<FixedBitSet>);

// impl Debug for Graph {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.debug_tuple("Graph")
//             .field(&self.0.iter().map(|x| format!("{}", x)).collect::<Vec<_>>())
//             .finish()
//     }
// }

#[derive(Debug)]
struct DirectoryTokens {
    file_token_indices_original: Vec<Vec<usize>>,
    tokens: Vec<(String, usize)>,
    // coloring: Vec<usize>,
    // color_to_tokens: Graph,
    tokens_sort: Vec<usize>,
    inverse_tokens_sort: Vec<usize>,
    color_bounds: Vec<usize>,
    color_count: usize,
    // token_graph: ,
    possible_files_bounds: Vec<usize>,
    num_of_colors: Vec<usize>,
    total_possible_files: usize,
}

impl DirectoryTokens {
    pub fn from_file_paths<P: AsRef<Path>>(file_paths: &Vec<P>) -> Result<Self> {
        // tokenise
        flame::start("tokenise");
        let mut file_word_counts = Vec::with_capacity(file_paths.len());
        for file_path in file_paths {
            let mut word_counts = HashMap::new();
            for token in file_path
                .as_ref()
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
        flame::end("tokenise");
        // get max counts
        flame::start("get max counts");
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
        flame::end("get max counts");
        flame::start("collection");
        let mut max_word_counts = max_word_counts.into_iter().collect::<Vec<_>>();
        max_word_counts.sort();
        // [(word, index), (word, index), ...]
        let tokens = max_word_counts
            .into_iter()
            .map(|(word, max_count)| (0..max_count).map(move |index| (word.clone(), index)))
            .flatten()
            .collect::<Vec<_>>();
        flame::end("collection");
        // token_to_index[(word, index)] = token_index
        flame::start("token to index");
        let mut token_to_index = HashMap::new();
        for (i, token) in tokens.iter().enumerate() {
            token_to_index.insert(token.clone(), i);
        }
        flame::end("token to index");
        // file_paths[i] <-> file_tokens[i] = FixedBitSet with contained token indices set
        flame::start("getting file token indices");
        let mut file_token_indices_original = Vec::with_capacity(file_word_counts.len());
        for counts in file_word_counts {
            let token_to_index = &token_to_index;
            let mut token_indices = Vec::new();
            for (word, &count) in &counts {
                token_indices
                    .extend((0..count).map(move |index| token_to_index[&(word.clone(), index)]));
            }
            file_token_indices_original.push(token_indices);
        }
        flame::end("getting file token indices");
        // undirected graph where there is an edge between tokens if they ever appear in the same file
        flame::start("creating token graph");
        let mut token_graph = vec![IndexSet::new(); tokens.len()];
        for token_index in 0..tokens.len() {
            for file_token_indices in &file_token_indices_original {
                if file_token_indices.contains(&token_index) {
                    token_graph[token_index].extend(file_token_indices);
                }
            }
        }
        flame::end("creating token graph");
        // color token graph to find sets of mutually exclusive tokens
        flame::start("coloring");
        let (color_count, coloring) = iter_graphs::graph_coloring(&token_graph, tokens.len());
        flame::end("coloring");
        flame::start("aob");
        let mut num_of_colors = vec![0usize; color_count];
        for &color in &coloring {
            num_of_colors[color] += 1;
        }
        let mut tokens_sort = (0..tokens.len()).collect::<Vec<_>>();
        tokens_sort.sort_by(|&token_index_0, &token_index_1| {
            num_of_colors[coloring[token_index_0]].cmp(&num_of_colors[coloring[token_index_1]])
        });
        let mut inverse_tokens_sort = vec![0; tokens_sort.len()];
        for (i, &j) in tokens_sort.iter().enumerate() {
            inverse_tokens_sort[j] = i;
        }
        num_of_colors.sort();
        let mut color_bounds = Vec::with_capacity(color_count + 1);
        let mut bound = 0;
        color_bounds.push(bound);
        for &num_of_color in &num_of_colors {
            bound += num_of_color;
            color_bounds.push(bound);
        }
        let mut possible_files_bounds = Vec::with_capacity(color_count + 1);
        let mut bound = 1;
        possible_files_bounds.push(bound);
        for &num_of_color in &num_of_colors {
            bound *= num_of_color + 1;
            possible_files_bounds.push(bound);
        }
        let total_possible_files = bound;
        flame::end("aob");
        Ok(Self {
            tokens,
            tokens_sort,
            inverse_tokens_sort,
            color_bounds,
            file_token_indices_original,
            color_count,
            possible_files_bounds,
            num_of_colors,
            total_possible_files,
        })
    }
    fn get_token(&self, token_index: usize) -> &(String, usize) {
        &self.tokens[self.tokens_sort[token_index]]
    }
    fn token_indices_of_color(&self, color: usize) -> Range<usize> {
        self.color_bounds[color]..self.color_bounds[color + 1]
    }
    fn file_tokens(&self) -> impl Iterator<Item = impl Iterator<Item = &(String, usize)>> {
        self.file_token_indices_original
            .iter()
            .map(|file_token_indices| file_token_indices.iter().map(|&x| &self.tokens[x]))
    }
    fn file_token_indices(&self) -> impl Iterator<Item = Vec<Option<usize>>> {
        self.file_token_indices_original
            .iter()
            .map(|file_token_indices| {
                let mut token_indices = file_token_indices
                    .iter()
                    .map(|&index| self.inverse_tokens_sort[index])
                    .collect::<Vec<_>>();
                token_indices.sort();
                let mut v = vec![None; self.color_count];
                let mut i = 0;
                let mut color = 0;
                while i < token_indices.len() && color < self.color_count {
                    if self
                        .token_indices_of_color(color)
                        .contains(&token_indices[i])
                    {
                        v[color] = Some(token_indices[i]);
                        i += 1;
                    }
                    color += 1;
                }
                v
            })
    }
    fn token_index_of_color(&self, color: usize, offset: usize) -> usize {
        self.color_bounds[color] + offset
    }
    fn possible_file(&self, index: usize) -> Vec<Option<usize>> {
        let mut possible_file = vec![None; self.color_count];
        for (color, (&possible_files_bound, &num_of_color)) in self
            .possible_files_bounds
            .iter()
            .zip(&self.num_of_colors)
            .enumerate()
        {
            let offset = (index / possible_files_bound) % (num_of_color + 1);
            if offset != 0 {
                possible_file[color] = Some(self.token_index_of_color(color, offset - 1));
            }
        }
        possible_file
    }
    fn possible_file_edges(&self, index: usize) -> impl Iterator<Item = usize> {
        (0..self.color_count).filter_map(move |color| {
            // let possible_files_bound = self.possible_files_bounds[color];
            // let num_of_color = self.num_of_colors[color];
            let x =
            (index % self.possible_files_bounds[color + 1]) / self.possible_files_bounds[color];
            if x != 0 {
                Some(index - x * self.possible_files_bounds[color])
            } else {
                None
            }
            // let offset = (index / possible_files_bound) % (num_of_color + 1);
            // if offset != 0 {
            //     Some(index / possible_files_bound - offset + index % possible_files_bound)
            // } else {
            //     None
            // }
        })
    }
    fn real_files_as_possible_file_indices(&self) -> impl Iterator<Item = usize> {
        self.file_token_indices().map(|x| {
            let mut index = 0;
            // dbg!(&x);
            for color in 0..self.color_count {
                // dbg!(&self.color_bounds[color]);
                if let Some(x) = x[color] {
                    index += (x - self.color_bounds[color] + 1) * self.possible_files_bounds[color];
                }
            }
            index
        })
    }
}

#[test]
fn directory_tokens_test() -> Result<()> {
    let mut file_paths = Vec::new();
    for dir_entry in std::fs::read_dir(
        &"../assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Default",
    )? {
        let path = dir_entry?.path();
        if path.is_file() {
            file_paths.push(path);
        }
    }
    // sort file paths as the order may affect predictions made
    file_paths.sort();
    let d = DirectoryTokens::from_file_paths(&file_paths)?;
    println!("{:?}", d);
    for i in d.file_tokens() {
        println!("{:?}", i.collect::<Vec<_>>());
    }
    println!("{:?}", d.file_token_indices().collect::<Vec<_>>());
    for n in 0..d.color_count {
        println!(
            "{:?}",
            d.token_indices_of_color(n)
                .map(|x| d.get_token(x))
                .collect::<Vec<_>>()
        );
    }
    // for n in (0..*d.possible_files_bounds.last().unwrap()) {
    for n in 0..100 {
        println!("{:?}", d.possible_file(n));
    }
    println!("{:?}", d.possible_files_bounds);
    for n in 0..100 {
        dbg!(n);
        println!("node = {:?}", d.possible_file(n));
        for edge in d.possible_file_edges(n) {
            dbg!(edge);
            println!("edge = {:?}", d.possible_file(edge));
        }
    }
    for x in d.real_files_as_possible_file_indices() {
        // dbg!(&x);
        let x = d.possible_file(x);
        // dbg!(&x);
        println!("{:?}", x.iter().map(|x| match x {
            Some(x) => &d.get_token(*x).0,
            None => "_"
        }).collect::<Vec<_>>());
    }

    Ok(())
}

struct DirectoryRepresentationIntermediary {
    dir_ident: proc_macro2::Ident,
    path: syn::LitStr,
    file_paths: Vec<PathBuf>,
    dir_paths: Vec<PathBuf>,

    directory_tokens: DirectoryTokens,

    // possible_files: Vec<Vec<Option<usize>>>,
    predictions: Vec<(usize, usize)>,
}

impl DirectoryRepresentationIntermediary {
    fn from_path<P: AsRef<Path>>(dir: P) -> Result<Self> {
        flame::start("all");
        flame::start("setup");
        flame::start("aob");
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
        flame::end("aob");
        flame::start("create directory_tokens");
        let directory_tokens = DirectoryTokens::from_file_paths(&file_paths)?;
        flame::end("create directory_tokens");
        flame::end("setup");

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
        // flame::start("creating possible files");
        // // create all possible files
        // let mut possible_files = VecDeque::new();
        // possible_files.push_front(Vec::with_capacity(directory_tokens.color_count));
        // while let Some(partial) = possible_files.pop_front() {
        //     if partial.len() == directory_tokens.color_count {
        //         possible_files.push_front(partial);
        //         break;
        //     }
        //     {
        //         let mut new_partial = partial.clone();
        //         new_partial.push(None);
        //         possible_files.push_back(new_partial);
        //     }
        //     for token_index in directory_tokens.token_indices_of_color(partial.len()) {
        //         let mut new_partial = partial.clone();
        //         new_partial.push(Some(token_index));
        //         possible_files.push_back(new_partial);
        //     }
        // }
        // flame::end("creating possible files");
        // let color_counts = (0..directory_tokens.color_count)
        //     .into_iter()
        //     .map(|color| {
        //         let r = directory_tokens.token_indices_of_color(color);
        //         r.end - r.start
        //     })
        //     .collect::<Vec<_>>();
        // let mut ns = Vec::with_capacity(directory_tokens.color_count + 1);
        // let mut n = 1;
        // for &color_count in &color_counts {
        //     ns.push(n);
        //     n *= color_count + 1;
        // }
        // ns.reverse();
        // let mut possible_files = vec![vec![None; directory_tokens.color_count]; n];
        // for i in 0..n {
        //     for (color, &count) in color_counts.iter().enumerate() {
        //         let x = i / ns[color];
        //         if x > 0 {
        //             possible_files[i][color] = Some(x + directory_tokens.color_bounds[color] - 1);
        //         }
        //     }
        // }
        flame::start("creating graphs");
        // // create edges from all possible files to possible files where one token has been added
        let mut graph = vec![Vec::new(); directory_tokens.total_possible_files];
        let mut transposed_graph = vec![Vec::new(); directory_tokens.total_possible_files];
        for i in 0..directory_tokens.total_possible_files {
            for e in directory_tokens.possible_file_edges(i) {
                graph[i].push(e);
                transposed_graph[e].push(i);
            }
        }
        flame::end("creating graphs");
        //     for i in 0..n {
        //         for color in 0..directory_tokens.color_count {
        //             let ci =
        //             let offset = ns[color + 1];
        //             if offset != 0 {
        //                 graph[i].push(i - offset);
        //                 transposed_graph[i - offset].push(i);
        //             }
        //         }
        //     }

        //     if (true ) {
        //     let mut i = 0;
        //     for (origin, edges) in graph[0..10].iter().enumerate() {
        //         for target in edges {
        //             let x = (possible_files[origin].iter().map(|x| match x {
        //                 Some(x) => format!("{}", directory_tokens.get_token(*x).0),
        //                 None => "_".to_string()
        //             }).join(" "), possible_files[*target].iter().map(|x| match x {
        //                 Some(x) => format!("{}", directory_tokens.get_token(*x).0),
        //                 None => "_".to_string()
        //             }).join(" "));
        //             dbg!(x, i);
        //             i += 1;
        //         }
        //     }
        // } else {
        // let mut i = 0;
        // while i < n {
        //     let x = possible_files[i]
        //         .iter()
        //         .map(|x| match x {
        //             Some(x) => format!("{}", directory_tokens.get_token(*x).0),
        //             None => "_".to_string(),
        //         })
        //         .join(" ");
        //     dbg!(i, x);
        //     i += 1;
        // }
        // dbg!(&color_counts);

        // let mut mods: Vec<Vec<usize>> = Vec::with_capacity(n);
        // for i in 0..n {
        //     let mut row = Vec::with_capacity(color_count + 1);
        //     for j in 0..=color_count {
        //         row.push(i % ns[j]);
        //     }
        //     mods.push(row);
        // }
        // for i in 0..n {
        //     for color in (0..color_count).rev() {
        //         let offset = mods[i][color + 1] - mods[i][color];
        //         if offset != 0 {
        //             let target = i - offset;
        //             graph[i].push(target);
        //             transposed_graph[target].push(i);
        //         }
        //     }
        // }
        // dbg!(graph.iter().map(|x| format!("{}", x)).collect::<Vec<_>>());

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
        flame::start("initial predictions");
        // create predictions for what the user might mean that remove certain tokens
        // by flood filling from real files to possible files with extra tokens

        // let mut token_to_index_in_colors = vec![0; tokens.len()];
        // for (color, tokens) in color_to_tokens.iter().enumerate() {
        //     for (i, token) in tokens.ones().enumerate() {
        //         token_to_index_in_colors[token] = i;
        //     }
        // }
        let mut initial_predictions = Vec::new();
        // let mut visited = IndexSet::new();
        let mut visited = FixedBitSet::with_capacity(directory_tokens.total_possible_files);
        for index in directory_tokens.real_files_as_possible_file_indices() {
            initial_predictions.push((index, index));
            visited.insert(index);
            // dbg!(index);
        }
        // for file_tokens in &file_tokens {
        //     let mut file_tokens_possible_file = vec![None; color_count];
        //     for file_token in file_tokens.ones() {
        //         file_tokens_possible_file[coloring[file_token]] = Some(file_token);
        //     }
        //     let mut m = 1;
        //     let mut index = 0;
        //     for color in (0..color_count).rev() {
        //         let count = color_to_tokens[color].count_ones(..);
        //         if let Some(file_token) = file_tokens_possible_file[color] {
        //             index += m * (token_to_index_in_colors[file_token] + 1);
        //         }
        //         m *= count + 1;
        //     }
        //     // NOTE: if files aren't deterministically ordered, predictions could change
        //     // NOTE: just sorting by file path for now (above)
        //     initial_predictions.push((index, index));
        //     visited.insert(index);
        // }
        flame::end("initial predictions");
        flame::start("traversing graph");
        // flood fill graph of possible files to create predictions
        flame::start("1");
        let mut i = 0;
        let mut predictions = initial_predictions.clone();
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
        flame::end("1");
        flame::start("2");
        predictions.extend(initial_predictions.clone().into_iter());
        let j = i;
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
        flame::end("2");

        flame::start("3");
        
        let mut i = j;
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

        
        Ok(Self {
            dir_ident,
            path,
            file_paths,
            dir_paths,
            // tokens,
            // file_tokens,
            // coloring,
            // color_count,
            // color_to_tokens,
            // graph_coloring,
            directory_tokens,

            // possible_files,
            predictions,
        })
    }

    fn to_token_stream(&self) -> Result<proc_macro2::TokenStream> {
        flame::start("to token stream");
        // create mutually exculsive enums using graph coloring
        flame::start("mx_enums");
        let mx_enums = (0..self.directory_tokens.color_count).map(|color| {
            let token_indices = self.directory_tokens.token_indices_of_color(color);
            let enum_name = quote::format_ident!("_MX_{}", color);
            let variants = token_indices
                .map(|token_index| token_to_ident(&self.directory_tokens.get_token(token_index)));
            quote::quote! {
                #[derive(Debug)]
                pub enum #enum_name {
                    #(#variants,)*
                }
            }
        }).collect::<Vec<_>>();
        flame::end("mx_enums");
        // create function from possible files to paths
        flame::start("file_function_arms");
        let mut file_function_arms = Vec::new();
        for (file_tokens, file_path) in self
            .directory_tokens
            .file_token_indices()
            .zip(&self.file_paths)
        {
            // let mut variants = vec![quote! { None }; self.color_count];
            // for token_index in file_tokens.ones() {
            //     let color = self.coloring[token_index];
            //     let enum_name = format_ident!("_MX_{}", color);
            //     let variant = token_to_ident(&self.tokens[token_index]);
            //     variants[color] = quote! { Some( #enum_name :: #variant ) };
            // }
            let variants =
                file_tokens
                    .iter()
                    .enumerate()
                    .map(|(color, file_token)| match file_token {
                        Some(file_token) => {
                            let enum_name = quote::format_ident!("_MX_{}", color);
                            let file_token = self.directory_tokens.get_token(*file_token);
                            let variant = token_to_ident(file_token);
                            quote::quote! { Some( #enum_name :: #variant ) }
                        }
                        None => quote::quote! { None },
                    });
            let file_path = syn::LitStr::new(
                file_path
                    .to_str()
                    .ok_or(anyhow!("Could not convert file path to str"))?,
                proc_macro2::Span::call_site(),
            );
            file_function_arms.push(quote::quote! { (#(#variants,)*) => Some( #file_path ) });
        }
        flame::end("file_function_arms");
        flame::start("function_input_type");
        let function_input_type = (0..self.directory_tokens.color_count).map(|i| {
            let enum_name = quote::format_ident!("_MX_{}", i);
            quote::quote! {
                Option< #enum_name >
            }
        });
        flame::end("function_input_type");
        flame::start("file_function");
        let file_function_input_type = function_input_type.clone();
        let file_function = quote::quote! {
            pub fn file(possible_file: (#(#file_function_input_type,)*)) -> Option<&'static str> {
                match possible_file {
                    #(#file_function_arms,)*
                    _ => None
                }
            }
        };
        flame::end("file_function");
        // create function that maps from possible files to actual files by removing tokens
        flame::start("predict_function_arms");
        flame::start("collection");
        let mut possible_actual_files = Vec::new();
        for (possible, actual) in &self.predictions {
            let possible_file = self.directory_tokens.possible_file(*possible);
            let actual_file = self.directory_tokens.possible_file(*actual);
            possible_actual_files.push((possible_file, actual_file));
        }
        flame::end("collection");
        flame::start("streaming");
        // slow section
        let mut predict_function_arms = Vec::new();
        for (possible_file, actual_file) in &possible_actual_files {
            // let mut poss = Vec::new();
            for (color, token_index) in possible_file.iter().enumerate() {
                match token_index {
                    Some(token_index) => {
                        // let x = proc_macro2::Ident::new("Some", Span::call_site()).into();
                        // predict_function_arms.extend(x);
                        predict_function_arms.push_str("Some(");
                        predict_function_arms.push_str(&format!("_MX_{}::", color));
                        predict_function_arms.push_str(&token_to_ident(&self.directory_tokens.get_token(*token_index)));
                        // predict_function_arms.push(Token![)]);
                    },
                    None => {
                        predict_function_arms.push_str("None");
                    }
                }
                predict_function_arms.push(Token![,]());
            }
            predict_function_arms.push_str(")=>(");
            for (color, token_index) in actual_file.iter().enumerate() {
                match token_index {
                    Some(token_index) => {
                        predict_function_arms.push_str("Some(");
                        predict_function_arms.push_str(&format!("_MX_{}::", color));
                        predict_function_arms.push_str(&token_to_ident(&self.directory_tokens.get_token(*token_index)));
                        // predict_function_arms.push(')');
                    },
                    None => {
                        predict_function_arms.push_str("None");
                    }
                }
                predict_function_arms.push(Token![,]);
            }
            predict_function_arms.push_str("),");
        }
        flame::end("streaming");
        flame::start("big parse");
        // dbg!(&predict_function_arms);
        // let predict_function_arms = syn::parse_str::<syn::Expr>(&predict_function_arms)?;
        // let predict_function_arms = proc_macro2::TokenStream::from_str(&predict_function_arms).expect("bad token stream from_str");
        flame::end("big parse");
        flame::end("predict_function_arms");
        flame::start("pft");
        let predict_function_input_type = function_input_type.clone();
        let predict_function = quote::quote! {
            pub fn predict(possible_file: (#(#predict_function_input_type,)*)) -> (#(#function_input_type,)*) {
                match possible_file {
                    #predict_function_arms
                }
            }
        };
        flame::end("pft");
        // process sub directories
        flame::start("sub");
        let mut submodules = Vec::new();
        for sub_dir in &self.dir_paths {
            submodules
            .push(DirectoryRepresentationIntermediary::from_path(sub_dir)?.to_token_stream()?);
        }
    
        let dir_ident = &self.dir_ident;
        let path = &self.path;
        flame::end("sub");
        // dbg!(mx_enums.iter().map(|s| s.to_string()).collect::<Vec<_>>());

        flame::end("to token stream");
        Ok(quote::quote! {
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
    proc_macro2::Ident::new(&base, Span::call_site())
}

fn token_to_ident(token: &(String, usize)) -> proc_macro2::Ident {
    let base = token
        .0
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>();
    // two tokens that have the same string should never be in the same enum as they will always be in the same file name
    let base = format!("_{}", base);
    proc_macro2::Ident::new(&base, Span::call_site())
    // syn::parse_str(&base).expect("Could not parse str")
}

// #[test]
// fn count_colors() -> Result<()> {
//     // let s = "../assets/bevy_input_prompts/kenney/kenney_input-prompts/Keyboard & Mouse/Double";
//     let s = "../assets/bevy_input_prompts/xelu/Xelu_Free_Controller&Key_Prompts/Xbox Series";
//     let x = DirectoryRepresentationIntermediary::from_path(s, |non_exclusive, num_tokens| {
//         let order = graph_operations::degeneracy_ordering(&non_exclusive, num_tokens);
//         graph_operations::greedy_coloring(&non_exclusive, &order, num_tokens)
//     })?;
//     println!("degeneracy ordering, greedy: {}", x.mx_count);
//     let x = DirectoryRepresentationIntermediary::from_path(s, |non_exclusive, num_tokens| {
//         graph_operations::dsatur(&non_exclusive, num_tokens)
//     })?;
//     println!("dsatur: {}", x.mx_count);
//     let x = DirectoryRepresentationIntermediary::from_path(s, |non_exclusive, num_tokens| {
//         graph_operations::color_graph(&non_exclusive, num_tokens)
//     })?;
//     println!("exact: {}", x.mx_count);
//     Ok(())
// }
