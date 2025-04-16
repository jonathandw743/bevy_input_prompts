use anyhow::{Result, anyhow};
use bit_set::BitSet;
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

fn mx_contains_edge(mx: &Vec<BitSet>, u: usize, v: usize) -> bool {
    !mx[u].contains(v) && !mx[v].contains(u)
}

/// Finds maximal cliques containing all the vertices in r, some of the
/// vertices in p, and none of the vertices in x.
fn bron_kerbosch_pivot(
    mx: &Vec<BitSet>,
    r: BitSet,
    mut p: BitSet,
    mut x: BitSet,
    n: usize,
    ignore: &HashSet<usize>,
) -> Vec<BitSet> {
    let mut cliques = Vec::with_capacity(1);
    if p.is_empty() {
        if x.is_empty() {
            cliques.push(r);
        }
        return cliques;
    }
    // pick the pivot u to be the vertex with max degree
    let u = p
        .iter()
        .max_by_key(|&v| {
            let mut neighbours = 0;
            for i in 0..n {
                if ignore.contains(&v) || ignore.contains(&i) {
                    continue;
                }
                if mx_contains_edge(mx, v, i) {
                    neighbours += 1;
                }
            }
            neighbours
        })
        .unwrap();
    let mut todo = p
        .iter()
        //skip neighbors of pivot
        .filter(|&v| {
            if ignore.contains(&v) {
                return false;
            }
            if u == v {
                return true;
            }
            !mx_contains_edge(mx, v, u)
        })
        .collect::<Vec<_>>();
    while let Some(v) = todo.pop() {
        let mut neighbors = BitSet::from_iter(0..n);
        neighbors.difference_with(&mx[v]);
        for ig in ignore {
            neighbors.remove(*ig);
        }

        p.remove(v);
        let mut next_r = r.clone();
        next_r.insert(v);

        let mut next_p = p.clone();
        next_p.intersect_with(&neighbors);

        let mut next_x = x.clone();
        next_x.intersect_with(&neighbors);

        cliques.extend(bron_kerbosch_pivot(mx, next_r, next_p, next_x, n, ignore));

        x.insert(v);
    }

    cliques
}

fn mx(bit_sets: &Vec<BitSet>) -> Vec<BitSet> {
    let mut mx = Vec::new();
    let mut n = 0;
    loop {
        let mut result = BitSet::new();
        for bit_set in bit_sets {
            if bit_set.contains(n) {
                result.union_with(bit_set);
            }
        }
        if result.is_empty() {
            break;
        }
        mx.push(result);
        n += 1;
    }
    let mut cliques = Vec::new();
    let mut i = 0;
    let mut ignore = HashSet::new();
    while i < n {
        let r = BitSet::new();
        let p = BitSet::from_iter(0..n);
        let x = BitSet::new();
        let c = bron_kerbosch_pivot(&mx, r, p, x, n, &ignore);
        dbg!(&c);
        let c = c
            .into_iter()
            .max_by_key(|c| c.len())
            .unwrap();
        i += c.len();
        dbg!(i, n);
        for a in &c {
            ignore.insert(a);
        }
        cliques.push(c);
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

    cliques
}

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
        let mut tokenised_file_names = Vec::new();
        let mut file_names = Vec::new();
        let mut submodules = Vec::new();
        for dir_entry in std::fs::read_dir(&dir)? {
            let path = dir_entry?.path();
            if path.is_file() {
                let tokenised_file_name = path
                    .file_stem()
                    .clone()
                    .ok_or(anyhow!("Could not get file stem"))?
                    .to_str()
                    .ok_or(anyhow!("Could not convert file stem to str"))?
                    .split("_")
                    .map(|x| x.to_owned())
                    .collect::<Vec<_>>();
                let file_name = path
                    .to_str()
                    .ok_or(anyhow!("Could not convert file path to str"))?
                    .to_owned();
                tokenised_file_names.push(tokenised_file_name);
                file_names.push(file_name);
            }
            if path.is_dir() {
                submodules.push(directory_representation_module(&path, ignore)?)
            }
        }
        // get unique tokens
        let mut tokens = Vec::new();
        for p in &tokenised_file_names {
            for a in p {
                tokens.push(a.clone());
            }
        }
        tokens.sort();
        tokens.dedup();
        let mut token_to_index = HashMap::new();
        for (i, token) in tokens.iter().enumerate() {
            token_to_index.insert(token.clone(), i);
        }
        // convert each file into a bit set that says whether the file name contains a given token
        // assumes there aren't file names that are made of the same set of tokens e.g. keyboard_w and w_keyboard
        let mut bit_sets = Vec::new();
        for tokenised_file_name in &tokenised_file_names {
            let mut bit_set = BitSet::new();
            for t in tokenised_file_name {
                bit_set.insert(token_to_index[t]);
            }
            bit_sets.push(bit_set);
        }
        dbg!(&tokens);
        dbg!(tokens.len());
        dbg!();
        let mut mx = mx(&bit_sets);
        // add any that aren't left in any mutually exclusive set
        // (they might be mutually exclusive with other tokens but they got taken by another group)

        // let mut created_tokens = Vec::new();
        // for mx in &mx {
        //     created_tokens.extend(mx.clone());
        // }
        // for i in 0..tokens.len() as u32 {
        //     if created_tokens.contains(&NodeIndex::from(i)) {
        //         continue;
        //     }
        //     let h = HashSet::from_iter([NodeIndex::from(i)]);
        //     mx.push(h);
        // }

        // create enums out of the mutually exlusive tokens
        let mut mx_enums = Vec::new();
        let enum_name_base = "_MX_";
        for (i, mx) in mx.iter().enumerate() {
            let mx_tokens = mx.iter().map(|index| tokens[index].clone());
            let enum_name = Ident::new(&format!("{}{}", enum_name_base, i), Span::call_site());
            let variants = mx_tokens.clone().map(|t| filename_to_variant(&t));
            let arms = variants.clone().zip(mx_tokens).map(|(variant, t)| {
                let lit = LitStr::new(&t, Span::call_site());
                quote! { Self::#variant => #lit }
            });
            mx_enums.push(quote! {
                pub enum #enum_name {
                    #(#variants,)*
                }
                impl #enum_name {
                    pub fn str(&self) -> &'static str {
                        match self {
                            #(#arms,)*
                        }
                    }
                }
            });
        }
        // dbg!(&tokens[65], &tokens[80]);
        // create the module
        return Ok(quote! {
            pub mod #dir_variant {
                pub const PATH: &'static str = #file_name;
                #(#mx_enums)*
                #(#submodules)*
            }
        });
        // let mut variants = Vec::new();
        // let mut arms = Vec::new();
        // for ni in toposort(&g, None)
        //     .map_err(|_| anyhow!("Could not topologically sort {:?}", dir.as_ref()))?
        // {
        //     let token = g
        //         .node_weight(ni)
        //         .ok_or(anyhow!("Could not map node index to token"))?;
        //     let variant = filename_to_variant(&token);
        //     let token_lit = syn::LitStr::new(&token, Span::call_site());
        //     variants.push(variant.clone());
        //     arms.push(quote! {
        //         Self::#variant => #token_lit
        //     });
        // }
        // return Ok(quote! {pub mod #dir_variant {
        //     pub const PATH: &'static str = #file_name;
        //     const x: usize = #n_lit;
        //     // pub enum Tokens {
        //     //     #(#variants,)*
        //     // }
        //     pub fn a_to_p(a: u128) -> &'static str {
        //         match a {
        //             #(#rules,)*
        //         }
        //     }
        //     // impl Tokens {
        //     //     // pub fn str(&self) -> &'static str {
        //     //     //     match self {
        //     //     //         #(#arms,)*
        //     //     //     }
        //     //     // }
        //     //     #(#all_tokens,)*
        //     // }
        //     #(#submodules)*
        // }});
    }
    Err(anyhow!(""))
    // Ok(if dir.as_ref().is_file() {
    //     let variant = filename_to_variant(
    //         dir.as_ref()
    //             .file_name()
    //             .ok_or(anyhow!("No file name"))?
    //             .to_str()
    //             .ok_or(anyhow!("Could not convert file name to str"))?,
    //     );
    //     let file_name = syn::LitStr::new(
    //         dir.as_ref()
    //             .strip_prefix(ignore)
    //             .expect("Could not strip prefix")
    //             .to_str()
    //             .expect("Could not convert file name to str"),
    //         proc_macro2::Span::call_site(),
    //     );
    //     quote! {
    //         pub const #variant: &'static str = #file_name;
    //     }
    // } else {
    //     let dir_variant = filename_to_variant(
    //         dir.as_ref()
    //             .file_name()
    //             .expect("Could not get file_name")
    //             .to_str()
    //             .expect("Could not convert filename to str"),
    //     );
    //     let file_name = syn::LitStr::new(
    //         dir.as_ref()
    //             .strip_prefix(ignore)
    //             .expect("Could not strip prefix")
    //             .to_str()
    //             .expect("Could not convert file name to str"),
    //         proc_macro2::Span::call_site(),
    //     );
    //     let mut submodules = Vec::new();
    //     for dir_entry in std::fs::read_dir(&dir)? {
    //         let dir_entry = dir_entry?;
    //         submodules.push(directory_representation_module(dir_entry.path(), ignore)?)
    //     }
    //     quote! {
    //         pub const #dir_variant: &'static str = #file_name;
    //         pub mod #dir_variant { #(#submodules)* }
    //     }
    // })
}

fn filename_to_variant(name: &str) -> proc_macro2::Ident {
    let base = name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>();
    // let base = if base
    //     .chars()
    //     .next()
    //     .expect("Could not get first character of variant")
    //     .is_numeric()
    // {
    //     format!("_{}", base)
    // } else {
    //     base
    // };
    let base = format!("_{}", base);
    syn::parse_str::<proc_macro2::Ident>(&base).unwrap()
}
