use anyhow::{Result, anyhow};
use fixedbitset::FixedBitSet;
use hashbrown::HashMap;
use itertools::Itertools;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use std::{
    borrow::Cow,
    collections::{HashSet, VecDeque},
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
    DirectoryRepresentationIntermediary::from_path(dir_path, |non_exclusive, num_tokens| {
        let order = graph_operations::degeneracy_ordering(&non_exclusive, num_tokens);
        graph_operations::greedy_coloring(&non_exclusive, &order, num_tokens)
    })
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
    predictions: Vec<(Vec<Option<usize>>, Vec<Option<usize>>)>,
    graph_coloring: fn(&Vec<FixedBitSet>, usize) -> (usize, Vec<usize>),
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
        // token_index -> color
        let (color_count, coloring) = graph_coloring(&token_graph, tokens.len());
        // color -> [token_index, token_index, ...]
        let mut color_to_tokens = vec![FixedBitSet::with_capacity(tokens.len()); color_count];
        for (token_index, &color) in coloring.iter().enumerate() {
            color_to_tokens[color].insert(token_index);
        }

        #[cfg(debug_assertions)]
        {
            let min_colors = file_tokens
                .iter()
                .map(|file_tokens| file_tokens.count_ones(..))
                .max()
                .unwrap_or(0);
            if min_colors != color_count {
                dbg!(dir.as_ref(), min_colors, color_count);
            }
            let mut x = 1;
            for c in &color_to_tokens {
                x *= c.count_ones(..) + 1;
            }
            dbg!(x);
        }

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

        let mut possible_file_tokens = Vec::new();
        let mut s = vec![Vec::with_capacity(color_count)];
        while let Some(x) = s.pop() {
            if x.len() == color_count {
                possible_file_tokens.push(x);
            } else {
                {
                    let mut x = x.clone();
                    x.push(None);
                    s.push(x);
                }
                for token_index in color_to_tokens[x.len()].ones() {
                    let mut x = x.clone();
                    x.push(Some(token_index));
                    s.push(x);
                }
            }
        }
        // dbg!(possible_file_tokens.iter().map(|x| {
        //     x.iter().map(|n| match n {
        //         Some(n) => format!("{}", n),
        //         None => "_".to_string(),
        //     }).join(" ")
        // }).collect::<Vec<_>>());
        

        // dbg!(color_to_tokens.iter().map(|x| format!("{}", x)).collect::<Vec<_>>());

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

        
        let mut predictions = Vec::new();

        // dbg!(possible_file_tokens.map(|x| format!("{}", x)).collect::<Vec<_>>());
        // for possible_file in &possible_files {
        //     for file_tokens in &file_tokens {}
        // }
        // for file_tokens in &file_tokens {
        //     let mut indices = vec![None; color_count];
        //     for token_index in file_tokens.ones() {
        //         indices[coloring[token_index]] = Some(token_index);
        //     }
        //     possible_files.push(indices);
        // }

        // flood fill graph of possible files to create predictions
        // let mut predictions = Vec::new();
        // let mut visited = HashSet::new();
        // let mut i = 0;
        // while i < predictions.len() {
        //     let file_node = predictions[i].clone();
        //     dbg!(&file_node);
        //     for (color, token_index) in file_node.0.iter().enumerate() {
        //         if token_index.is_some() {
        //             continue;
        //         }
        //         for &token_index in &color_to_token_indices[coloring[color]] {
        //             let mut new_file_node = file_node.clone();
        //             new_file_node.0[color] = Some(token_index);
        //             if !visited.contains(&new_file_node.0) {
        //                 visited.insert(new_file_node.0.clone());
        //                 predictions.push(new_file_node);
        //             }
        //         }
        //     }
        //     i += 1;
        // }

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
            predictions,
            graph_coloring,
        })
    }

    fn to_token_stream(&self) -> Result<proc_macro2::TokenStream> {
        // create mutually exculsive enums using graph coloring
        let mx_enums =
            self.color_to_tokens
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
        let mut function_arms = Vec::new();
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
            function_arms.push(quote! { (#(#variants,)*) => Some( #file_path ) });
        }
        let function_input_type = (0..self.color_count).map(|i| {
            let enum_name = format_ident!("_MX_{}", i);
            quote! {
                Option< #enum_name >
            }
        });
        let function = quote! {
            pub fn file(possible_file: (#(#function_input_type,)*)) -> Option<&'static str> {
                match possible_file {
                    #(#function_arms,)*
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
                #function
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
