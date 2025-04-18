use anyhow::{Result, anyhow};
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use bit_set::BitSet;
use fixedbitset::FixedBitSet;
use hashbrown::{HashMap, HashSet};
// use petgraph::{
//     Graph,
//     algo::{maximal_cliques, toposort},
//     data::Build,
//     graph::{NodeIndex, UnGraph},
//     visit::{GetAdjacencyMatrix, IntoNeighbors},
// };
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{TokenStreamExt, format_ident, quote};
use std::{hash::Hash, path::Path};
use syn::{
    Expr, Ident, LitBool, LitInt, LitStr, Token,
    parse::{self, Parse},
    parse_macro_input, parse2,
    punctuated::Punctuated,
    token::{Comma, Struct},
};

#[proc_macro]
pub fn directory_representation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr).value();
    let dir_path = Path::new(&input);
    directory_representation_module(dir_path, dir_path.parent().unwrap_or(Path::new("")))
        .expect("Could not create directory representation module")
        .into()
}


fn non_exclusive(bit_sets: &Vec<FixedBitSet>, n: usize) -> Vec<FixedBitSet> {
    let mut non_exclusive = vec![FixedBitSet::with_capacity(n); n];
    for bit in 0..n {
        for bit_set in bit_sets {
            if bit_set.contains(bit) {
                non_exclusive[bit].union_with(bit_set);
            }
        }
    }
    non_exclusive
}


/// Compute degeneracy ordering using a simple greedy peeling algorithm
fn degeneracy_ordering(graph: &Vec<FixedBitSet>, n: usize) -> Vec<usize> {
    let mut degrees: Vec<usize> = graph.iter().map(|neighbors| neighbors.count_ones(..)).collect();
    let mut order = Vec::with_capacity(n);
    let mut removed = FixedBitSet::with_capacity(n);

    for _ in 0..n {
        // Find vertex with smallest degree not yet removed
        let mut min_deg = usize::MAX;
        let mut min_v = None;
        for v in 0..n {
            if !removed.contains(v) && degrees[v] < min_deg {
                min_deg = degrees[v];
                min_v = Some(v);
            }
        }

        if let Some(v) = min_v {
            order.push(v);
            removed.insert(v);
            // Update degree of neighbors
            for u in graph[v].ones() {
                if !removed.contains(u) {
                    degrees[u] -= 1;
                }
            }
        }
    }

    order.reverse(); // For coloring, reverse it (lowest-degree last)
    order
}

fn greedy_coloring(graph: &Vec<FixedBitSet>, order: &[usize], n: usize) -> (usize, Vec<usize>) {
    let mut coloring = vec![usize::MAX; n];

    for &v in order {
        let mut used = FixedBitSet::with_capacity(n);
        for u in graph[v].ones() {
            if coloring[u] != usize::MAX {
                used.insert(coloring[u]);
            }
        }

        // Find the first available color
        let color = (0..n).find(|c| !used.contains(*c)).expect("color not available");
        coloring[v] = color;
    }

    let max_color: usize = coloring.iter().max().map_or(0, |x| x + 1);
    (max_color, coloring) // colors are 0-based
}

fn directory_representation_module<P: AsRef<Path>>(
    dir: P,
    ignore: &Path,
) -> Result<proc_macro2::TokenStream> {
    if dir.as_ref().is_dir() {
        let dir_variant = filename_to_variant(
            dir.as_ref()
                .file_name()
                .ok_or(anyhow!("Could not get file_name"))?
                .to_str()
                .ok_or(anyhow!("Could not convert filename to str"))?,
        );
        let file_name = syn::LitStr::new(
            dir.as_ref()
                //    .strip_prefix(ignore)?
                .to_str()
                .ok_or(anyhow!("Could not convert file name to str"))?,
            proc_macro2::Span::call_site(),
        );
        // read all file names and split them
        let mut token_set_to_file_name = Vec::new();
        // all tokens
        let mut tokens = Vec::new();
        // modules corresponding to subdirectories
        let mut submodules = Vec::new();
        for dir_entry in std::fs::read_dir(&dir)? {
            let path = dir_entry?.path();
            if path.is_file() {
                let mut tokenised_file_stem = path
                    .file_stem()
                    .ok_or(anyhow!("Could not get file stem"))?
                    .to_str()
                    .ok_or(anyhow!("Could not convert file stem to str"))?
                    .split("_")
                    .map(|x| x.to_owned())
                    .collect::<Vec<_>>();
                tokenised_file_stem.sort();
                tokenised_file_stem.dedup();
                let file_name = path
                    .to_str()
                    .ok_or(anyhow!("Could not convert file path to str"))?
                    .to_owned();
                token_set_to_file_name.push((tokenised_file_stem.clone(), file_name));
                tokens.extend(tokenised_file_stem);
            }
            if path.is_dir() {
                submodules.push(directory_representation_module(&path, ignore)?)
            }
        }
        // make unique (not using a hash set because i want a deterministic iteration)
        tokens.sort();
        tokens.dedup();
        // create a map from tokens to indices in tokens
        let mut token_to_index = HashMap::new();
        for (i, token) in tokens.iter().enumerate() {
            token_to_index.insert(token.clone(), i);
        }

        let num_files = token_set_to_file_name.len();
        let num_tokens = tokens.len();
        // convert each file into a bit set that says whether the file name contains a given token
        // assumes there aren't file names that are made of the same set of tokens e.g. keyboard_w and w_keyboard
        let mut bit_sets = vec![FixedBitSet::with_capacity(num_tokens); num_files];
        for (i, (token_set, file_name)) in token_set_to_file_name.iter().enumerate() {
            for token in token_set {
                bit_sets[i].insert(token_to_index[token]);
            }
        }
        let non_exclusive = non_exclusive(&bit_sets, num_tokens);

        dbg!(non_exclusive.iter().map(|v| format!("{}", v)).collect::<Vec<_>>());

        let order = degeneracy_ordering(&non_exclusive, num_tokens);
        let (mx_count, coloring) = greedy_coloring(&non_exclusive, &order, num_tokens);
        
        let mx_enum_names: Vec<_> = (0..mx_count).map(|color| format_ident!("_MX_{}", color)).collect();
        let mut mx_enum_variants = vec![Vec::new(); mx_count];
        for (&color, token) in coloring.iter().zip(tokens) {
            mx_enum_variants[color].push(filename_to_variant(&token));
        }
        let mut mx_enums = mx_enum_names.iter().zip(mx_enum_variants).map(|(enum_name, variants)| quote! {
            pub enum #enum_name {
                #(#variants,)*
            }
        });
        // for (i, mutually_exclusive_token_indices) in cliques.iter().enumerate() {
        //     let mutually_exclusive_tokens = mutually_exclusive_token_indices.ones().map(|index| tokens[index].clone()).collect::<Vec<_>>();
        //     let enum_name = Ident::new(&format!("_MX_{}", i), Span::call_site());
        //     let variants = mutually_exclusive_tokens.clone().into_iter().map(|token| filename_to_variant(&token)).collect::<Vec<_>>();
        //     // let str_arms = variants.clone().into_iter().zip(mutually_exclusive_tokens.clone()).map(|(variant, token)| {
        //     //     let lit = LitStr::new(&token, Span::call_site());
        //     //     quote! { Self::#variant => #lit }
        //     // });
        //     mx_enums.push(quote! {
        //         pub enum #enum_name {
        //             #(#variants,)*
        //         }
        //         // impl #enum_name {
        //         //     pub fn str(&self) -> &'static str {
        //         //         match self {
        //         //             #(#str_arms,)*
        //         //         }
        //         //     }
        //         // }
        //     });
        //     for (token, variant) in mutually_exclusive_tokens.into_iter().zip(variants) {
        //         token_to_enum_name.insert(token, (i, quote!{ #enum_name::#variant }));
        //     }
        // }
        let mut function_arms = Vec::with_capacity(num_files);
        for (i, (token_set, file_name)) in token_set_to_file_name.iter().enumerate() {
            let mut variant_exprs = vec![None; mx_count];
            for token in token_set {
                let color = coloring[token_to_index[token]];
                let enum_name = &mx_enum_names[color];
                let variant = filename_to_variant(&token);
                variant_exprs[color] = Some(quote!{ #enum_name::#variant });
            }
            let variant_exprs_unwrap = variant_exprs.into_iter().map(|v| {
                match v {
                    Some(v) => quote! { Some(#v) },
                    None => quote! { None },
                }
            });
            let lit = LitStr::new(file_name, Span::call_site());
            function_arms.push(quote! { (#(#variant_exprs_unwrap,)*) => Some(#lit) });
        }
        let tctype = (0..mx_count).map(|i| {
            let e = Ident::new(&format!("_MX_{}", i), Span::call_site());
            quote! {
                Option<#e>
            }
        });
        let function = if num_files == 0 {
            quote! {}
        } else {
            quote! {
                pub fn file(tc: (#(#tctype,)*)) -> Option<&'static str> {
                    match tc {
                        #(#function_arms,)*
                        _ => None
                    }
                }
            }
        };
        return Ok(quote! {
            pub mod #dir_variant {
                pub const PATH: &'static str = #file_name;
                #(#mx_enums)*
                #(#submodules)*
                #function
            }
        });
    }
    Err(anyhow!(""))
}

fn filename_to_variant(name: &str) -> proc_macro2::Ident {
    let base = name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>();
    let base = format!("_{}", base);
    syn::parse_str::<proc_macro2::Ident>(&base).unwrap()
}
