use anyhow::{Result, anyhow};
use bit_set::BitSet;
use fixedbitset::FixedBitSet;
use hashbrown::{HashMap, HashSet};
use petgraph::{
    Graph,
    algo::{maximal_cliques, toposort},
    data::Build,
    graph::{NodeIndex, UnGraph},
    visit::{GetAdjacencyMatrix, IntoNeighbors},
};
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

// struct BigUint<T>(T);

// #[proc_macro]
// pub fn big_int(len: TokenStream) -> TokenStream {
//     let len = parse_macro_input!(len as LitInt);
//     let Ok(len) = len.base10_parse::<usize>() else {
//         return syn::Error::new_spanned(len, "Cannot parse input")
//             .to_compile_error()
//             .into();
//     };
//     let struct_name = format_ident!("BigInt{}", len);
//     let mut tuple_type = Vec::new();
//     let u128_count = len / 128;
//     for _ in 0..u128_count {
//         tuple_type.push(quote! { u128 });
//     }
//     let extra = len - u128_count;
//     match extra {
//         0 => {}
//         1..=8 => {
//             tuple_type.push(quote! { u8 });
//         }
//         9..=16 => {
//             tuple_type.push(quote! { u16 });
//         }
//         17..=32 => {
//             tuple_type.push(quote! { u32 });
//         }
//         33..=64 => {
//             tuple_type.push(quote! { u64 });
//         }
//         65..=127 => {
//             tuple_type.push(quote! { u128 });
//         }
//         128.. => unreachable!(),
//     }
//     let struct_def = quote! { struct #struct_name (#(#tuple_type,)*); };
//     let impl_block = quote! {
//         impl #struct_name {
//             pub fn zero() -> Self {
//                 return
//             }
//             pub fn from_one(shift: usize) -> Self {

//             }
//             pub fn or(&self, other: Self) -> Self {

//             }
//         }
//     }
// }
// struct Input {
//     x: Punctuated::<Expr, Token![,]>,
// }

// impl Parse for Input {
//     fn parse(input: parse::ParseStream) -> syn::Result<Self> {
//         let f =
//     }
// }

// struct I(Punctuated<Expr, Token![,]>);

// impl Parse for I {
//     fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
//         Ok(I(input.parse_terminated(Expr::parse, Token![,])?))
//     }
// }

// #[proc_macro]
// pub fn show_expr(input: TokenStream) -> TokenStream {
//     let exprs = parse_macro_input!(input as I);
//     let mut exprs: Vec<_> = exprs.0.into_iter().collect();
//     exprs.sort();

//     quote! {}.into()
// }

// #[proc_macro]
// pub fn foo(input: TokenStream) -> TokenStream {
//     let input = parse_macro_input!(input as LitStr).value();

// }

#[proc_macro]
pub fn directory_representation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr).value();
    let dir_path = Path::new(&input);
    directory_representation_module(dir_path, dir_path.parent().unwrap_or(Path::new("")))
        .expect("Could not create directory representation module")
        .into()
}

// fn directory_representation_module<P: AsRef<Path>>(
//     dir: P,
//     ignore: &Path,
// ) -> Result<proc_macro2::TokenStream> {
//     Ok(if dir.as_ref().is_file() {
//         let variant = filename_to_variant(
//             dir.as_ref()
//                 .file_name()
//                 .ok_or(anyhow!("No file name"))?
//                 .to_str()
//                 .ok_or(anyhow!("Could not convert file name to str"))?,
//         );
//         let file_name = syn::LitStr::new(
//             dir.as_ref()
//                 .strip_prefix(ignore)
//                 .expect("Could not strip prefix")
//                 .to_str()
//                 .expect("Could not convert file name to str"),
//             proc_macro2::Span::call_site(),
//         );
//         quote! {
//             pub const #variant: &'static str = #file_name;
//         }
//     } else {
//         let dir_variant = filename_to_variant(
//             dir.as_ref()
//                 .file_name()
//                 .expect("Could not get file_name")
//                 .to_str()
//                 .expect("Could not convert filename to str"),
//         );
//         let file_name = syn::LitStr::new(
//             dir.as_ref()
//                 .strip_prefix(ignore)
//                 .expect("Could not strip prefix")
//                 .to_str()
//                 .expect("Could not convert file name to str"),
//             proc_macro2::Span::call_site(),
//         );
//         let mut submodules = Vec::new();
//         for dir_entry in std::fs::read_dir(&dir)? {
//             let dir_entry = dir_entry?;
//             submodules.push(directory_representation_module(dir_entry.path(), ignore)?)
//         }
//         quote! {
//             pub const #dir_variant: &'static str = #file_name;
//             pub mod #dir_variant { #(#submodules)* }
//         }
//     })
// }

// fn are_mutually(mx: &Vec<BitSet>, u: usize, v: usize) -> bool {
//     !mx[u].contains(v) && !mx[v].contains(u)
// }

/// Finds maximal cliques containing all the vertices in r, some of the
/// vertices in p, and none of the vertices in x.
fn bron_kerbosch_pivot(
    non_exclusive: &Vec<FixedBitSet>,
    r: FixedBitSet,
    mut p: FixedBitSet,
    mut x: FixedBitSet,
    n: usize,
    // ignore: &HashSet<usize>,
) -> Vec<FixedBitSet> {
    let mut cliques = Vec::with_capacity(1);
    if p.is_clear() {
        if x.is_clear() {
            cliques.push(r);
        }
        return cliques;
    }
    // pick the pivot u to be the vertex with max degree
    // println!("{}", p);
    let u = p.ones()
        .max_by_key(|&v| {
            let mut neighbours = 0;
            for i in 0..n {
                // if ignore.contains(&v) || ignore.contains(&i) {
                //     continue;
                // }
                if !non_exclusive[v].contains(i) && !non_exclusive[i].contains(v) {
                    neighbours += 1;
                }
                // if mx_contains_edge(mx, v, i) {
                //     neighbours += 1;
                // }
            }
            neighbours
        })
        .expect("there should be a vertex with max degree");
    let mut todo = p
        .ones()
        //skip neighbors of pivot
        .filter(|&v| {
            // if ignore.contains(&v) {
            //     return false;
            // }
            if u == v {
                return true;
            }
            !(!non_exclusive[u].contains(v) && !non_exclusive[v].contains(u))
        })
        .collect::<Vec<_>>();
    while let Some(v) = todo.pop() {
        let mut neighbors = FixedBitSet::from_iter(0..n);
        neighbors.difference_with(&non_exclusive[v]);
        // for ig in ignore {
        //     neighbors.remove(*ig);
        // }

        p.remove(v);
        let mut next_r = r.clone();
        next_r.insert(v);

        let mut next_p = p.clone();
        next_p.intersect_with(&neighbors);

        let mut next_x = x.clone();
        next_x.intersect_with(&neighbors);

        cliques.extend(bron_kerbosch_pivot(non_exclusive, next_r, next_p, next_x, n));

        x.insert(v);
    }

    cliques
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
fn cliques(non_exclusive: &Vec<FixedBitSet>, n: usize) -> Vec<FixedBitSet> {
    let mut cliques = Vec::new();
    let mut i = 0;
    let mut p = FixedBitSet::from_iter(0..n);
    while i < n {
        let r = FixedBitSet::with_capacity(n);
        let x = FixedBitSet::with_capacity(n);
        let c = bron_kerbosch_pivot(&non_exclusive, r, p.clone(), x, n);
        let c = c.into_iter().max_by_key(|c| c.count_ones(..)).unwrap();
        i += c.count_ones(..);
        for a in c.ones() {
            p.remove(a);
        }
        cliques.push(c);
    }
    cliques
}

//    let mut collected = HashSet::new();
//    for clique in &cliques {
//        for i in clique {
//            collected.insert(i);
//        }
//    }
//    for i in 0..n {
//        if !collected.contains(&i) {
//            dbg!(i);
//        }
//    }

// #[test]
// fn test_mx() {
//     let bit_sets = vec![
//         BitSet::from_bytes(&[0b10100000]),
//         BitSet::from_bytes(&[0b01100000]),
//         BitSet::from_bytes(&[0b00100000]),
//         BitSet::from_bytes(&[0b00010000]),
//     ];
//     let mx = mx(bit_sets);
//     let mut edges = Vec::new();
//     for i in 0..mx.len() {
//         for j in (i + 1)..mx.len() {
//             if !mx[i].contains(j) && !mx[j].contains(i) {
//                 edges.push((i as u32, j as u32));
//             }
//         }
//     }
//     let mut g = UnGraph::<usize, ()>::from_edges(edges);
//     dbg!(maximal_cliques(&g));
// }

fn is_valid(u: usize, color: usize, coloring: &[usize], graph: &[FixedBitSet]) -> bool {
    for &v in graph[u].ones() {
        if coloring[v] == color {
            return false;
        }
    }
    true
}

fn color_graph(
    u: usize,
    max_color: usize,
    coloring: &mut [usize],
    graph: &[FixedBitSet],
) -> bool {
    if u == graph.len() {
        return true;
    }

    for color in 0..max_color {
        if is_valid(u, color, coloring, graph) {
            coloring[u] = color;
            if color_graph(u + 1, max_color, coloring, graph) {
                return true;
            }
            coloring[u] = usize::MAX; // reset
        }
    }

    false
}

fn min_coloring(graph: &[FixedBitSet]) -> Vec<usize> {
    let n = graph.len();
    for k in 1..=n {
        let mut coloring = vec![usize::MAX; n];
        if color_graph(0, k, &mut coloring, graph) {
            return coloring;
        }
    }
    unreachable!()
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
        let cliques = cliques(&non_exclusive, num_tokens);
        let mut token_to_enum_name = HashMap::new(); 
        // create enums out of the mutually exlusive tokens
        let mut mx_enums = Vec::new();
        for (i, mutually_exclusive_token_indices) in cliques.iter().enumerate() {
            let mutually_exclusive_tokens = mutually_exclusive_token_indices.ones().map(|index| tokens[index].clone()).collect::<Vec<_>>();
            let enum_name = Ident::new(&format!("_MX_{}", i), Span::call_site());
            let variants = mutually_exclusive_tokens.clone().into_iter().map(|token| filename_to_variant(&token)).collect::<Vec<_>>();
            // let str_arms = variants.clone().into_iter().zip(mutually_exclusive_tokens.clone()).map(|(variant, token)| {
            //     let lit = LitStr::new(&token, Span::call_site());
            //     quote! { Self::#variant => #lit }
            // });
            mx_enums.push(quote! {
                pub enum #enum_name {
                    #(#variants,)*
                }
                // impl #enum_name {
                //     pub fn str(&self) -> &'static str {
                //         match self {
                //             #(#str_arms,)*
                //         }
                //     }
                // }
            });
            for (token, variant) in mutually_exclusive_tokens.into_iter().zip(variants) {
                token_to_enum_name.insert(token, (i, quote!{ #enum_name::#variant }));
            }
        }
        let mut function_arms = Vec::with_capacity(num_files);
        for (i, (token_set, file_name)) in token_set_to_file_name.iter().enumerate() {
            let mut variant_exprs = vec![None; cliques.len()];
            for token in token_set {
                let (clique_index, variant_expr) = &token_to_enum_name[token];
                variant_exprs[*clique_index] = Some(variant_expr);
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
        let tctype = (0..cliques.len()).map(|i| {
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
