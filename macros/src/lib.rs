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
        let file_tokens = file_word_counts
            .into_iter()
            .map(move |counts| {
                let token_to_index = &token_to_index;
                FixedBitSet::from_iter(
                    counts
                        .into_iter()
                        .map(move |(word, max_count)| {
                            let token_to_index = token_to_index;
                            (0..max_count).map(move |index| token_to_index[&(word.clone(), index)])
                        })
                        .flatten(),
                )
            })
            .collect::<Vec<_>>();
        // undirected graph where there is an edge between tokens if they ever appear in the same file
        let mut token_graph = vec![FixedBitSet::with_capacity(tokens.len()); tokens.len()];
        for token_index in 0..tokens.len() {
            for file_tokens in &file_tokens {
                if file_tokens.contains(token_index) {
                    token_graph[token_index].union_with(file_tokens);
                }
            }
        }
        // color token graph to find sets of mutually exclusive tokens
        let (color_count, coloring) = graph_coloring(&token_graph, tokens.len());
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
        //     if min_colors != color_count {
        //         dbg!(dir.as_ref(), min_colors, color_count);
        //     }
        //     let mut x = 1;
        //     for c in &color_to_tokens {
        //         x *= c.count_ones(..) + 1;
        //     }
        //     dbg!(x);
        // }

        // let mut possible_files = Vec::new();
        // let mut function = |x| {
        //     possible_files.push(x);
        // };
        // for (color, token_indices) in color_to_token_indices.iter().enumerate() {
        //     function = |x| {
        //         for token_index in token_indices {
        //             let mut x = x.clone();
        //             x.push(token_index);
        //             function(x);
        //         }
        //     };
        // }
        // function(Vec::new());
        // dbg!(possible_files);

        // possibly inefficient, should create my own multi_cartesian product

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
        // create edges from all possible files to possible files where one token has been added
        let mut n = 1;
        for c in &color_to_tokens {
            n *= c.count_ones(..) + 1;
        }
        let mut edges = Vec::new();
        let mut m = 1;
        for i in (0..color_count).rev() {
            let count = color_to_tokens[i].count_ones(..);
            for j in (0..n).step_by(m * (count + 1)) {
                for k in (0..(m * (count + 1))).step_by(m) {
                    // no edge to self
                    if k == 0 {
                        continue;
                    }
                    edges.push((j, j + k));
                }
            }
            m *= count + 1;
        }
        // create that graph
        // edges that represent a low information token being added should appear first
        let mut graph =
            vec![FixedBitSet::with_capacity(possible_files.len()); possible_files.len()];
        for edge in edges {
            graph[edge.0].insert(edge.1);
        }

        // let queue =

        // let node = 9000;
        // let edges = &graph[node];
        // for edge in edges.ones() {
        //     dbg!(node, edge);
        //     let x = queue[node]
        //         .iter()
        //         .map(|n| match n {
        //             Some(n) => format!("{:?}", tokens[*n]),
        //             None => "_".to_string(),
        //         })
        //         .join(" ");
        //     dbg!(x);
        //     let x = queue[edge]
        //         .iter()
        //         .map(|n| match n {
        //             Some(n) => format!("{:?}", tokens[*n]),
        //             None => "_".to_string(),
        //         })
        //         .join(" ");
        //     dbg!(x);
        // }

        // let mut boundary = 1;
        // for (i, token_count) in color_to_tokens.iter().map(|token_indices| token_indices.count_ones(..) + 1).enumerate() {

        //     boundary *= token_count;
        // }

        // let mut boundaries = vec![1];
        // for token_indices in color_to_tokens {
        //     let lower
        //     boundaries.push(boundaries.last().unwrap() * token_indices.count_ones(..));
        // }

        // dbg!(edges.iter().map(|x| format!("{} {}", x.0, x.1)).collect::<Vec<_>>());

        // let mut graph = vec![
        //     FixedBitSet::with_capacity(possible_file_tokens.len());
        //     possible_file_tokens.len()
        // ];
        // for (origin, target) in edges {
        //     graph[origin].insert(target);
        // }

        // dbg!(
        //     queue
        //         .iter()
        //         .map(|x| {
        //             x.iter()
        //                 .map(|n| match n {
        //                     Some(n) => format!("{:?}", tokens[*n]),
        //                     None => "_".to_string(),
        //                 })
        //                 .join(" ")
        //         })
        //         .collect::<Vec<_>>()
        // );

        // for (origin, target) in &edges { // [0..100] {
        //     let x = queue[*origin]
        //         .iter()
        //         .map(|n| match n {
        //             Some(n) => format!("{:?}", tokens[*n]),
        //             None => "_".to_string(),
        //         })
        //         .join(" ");
        //     dbg!(x);

        //     let x = queue[*target]
        //         .iter()
        //         .map(|n| match n {
        //             Some(n) => format!("{:?}", tokens[*n]),
        //             None => "_".to_string(),
        //         })
        //         .join(" ");
        //     dbg!(x);
        //     dbg!("---");
        // }

        // dbg!(
        //     graph
        //         .iter()
        //         .map(|x| format!("{}", x))
        //         .collect::<Vec<_>>()
        // );

        // dbg!(
        //     graph
        //         .iter()
        //         .map(|x| x.ones().map(|x| format!("{}", x)).join(" "))
        //         .collect::<Vec<_>>()
        // );

        // dbg!(&real_file_indices);

        // dbg!(graph);

        // let possible_file_tokens = color_to_tokens
        //     .iter()
        //     .map(|token_indices| {
        //         once(None)
        //             .chain(token_indices.ones().map(|token_index| Some(token_index)))
        //             .collect::<Vec<_>>()
        //     })
        //     .multi_cartesian_product()
        //     .map(|token_indices| {
        //         FixedBitSet::from_iter(
        //             token_indices
        //                 .into_iter()
        //                 .filter_map(|token_index| token_index),
        //         )
        //     });

        // let mut possible_files_graph = HashMap::new();

        // let mut predictions = Vec::new();

        // dbg!(possible_file_tokens.map(|x| format!("{}", x)).collect::<Vec<_>>());
        // for possible_file in &possible_files {
        //     for file_tokens in &file_tokens {}
        // }
        let mut predictions = Vec::new();
        let mut visited = FixedBitSet::with_capacity(possible_files.len());
        for file_tokens in &file_tokens {
            let mut m = 1;
            // let mut indices = vec![None; color_count];
            let mut index = 0;
            for i in 1..=color_count {
                let color = color_count - i;
                let count = color_to_tokens[color].count_ones(..);
                // for token_index in file_tokens.ones() {
                //     indices[coloring[token_index]] = Some(token_index);
                // }
                // possible_files.push(indices);
                for file_token in file_tokens.ones() {
                    for (j, token) in color_to_tokens[color].ones().enumerate() {
                        if file_token == token {
                            index += m * (j + 1);
                        }
                    }
                }
                m *= count + 1;
            }
            // NOTE: if files aren't deterministically ordered, predictions could change
            // NOTE: just sorting by file path for now
            // dbg!(file_tokens.ones().collect::<Vec<_>>());
            // dbg!(file_tokens.ones().map(|x| &tokens[x]).collect::<Vec<_>>());
            // dbg!(&index);
            predictions.push((index, index));
            visited.insert(index);
            // dbg!(&possible_files[index]);
            // dbg!(&graph[index].ones().collect::<Vec<_>>());
            // dbg!("-----");
        }

        // flood fill graph of possible files to create predictions
        // let mut predictions = Vec::new();
        let mut i = 0;
        while i < predictions.len() {
            // if visited.contains(predictions[i].0) {
            //     continue;
            // }
            // visited.insert(predictions[i].0);
            for edge in graph[predictions[i].0].ones() {
                if visited.contains(edge) {
                    continue;
                }
                visited.insert(edge);
                predictions.push((edge, predictions[i].1));
            }
            // let file_node = predictions[i].clone();
            // dbg!(&file_node);
            // for (color, token_index) in predictions[i].0.iter().enumerate() {
            //     if token_index.is_some() {
            //         continue;
            //     }
            //     for &token_index in &color_to_token_indices[coloring[color]] {
            //         let mut new_file_node = file_node.clone();
            //         new_file_node.0[color] = Some(token_index);
            //         if !visited.contains(&new_file_node.0) {
            //             visited.insert(new_file_node.0.clone());
            //             predictions.push(new_file_node);
            //         }
            //     }
            // }

            i += 1;
        }
        // dbg!(predictions);
        // dbg!(&possible_files[125]);
        // dbg!(&possible_files[124]);
        // dbg!(&tokens[0]);
        // dbg!(&tokens[65]);
        // dbg!(&tokens[13]);

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
                (#(#possible_variants,)*) => Some( (#(#actual_variants,)*) )
            });
        }
        let predict_function_input_type = function_input_type.clone();
        let predict_function = quote! {
            pub fn predict(possible_file: (#(#predict_function_input_type,)*)) -> Option<(#(#function_input_type,)*)> {
                match possible_file {
                    #(#predict_function_arms,)*
                    _ => None
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
