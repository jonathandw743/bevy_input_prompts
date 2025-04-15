use anyhow::{Result, anyhow};
use bit_set::BitSet;
use hashbrown::{HashMap, HashSet};
use petgraph::{
    Graph,
    algo::{maximal_cliques, toposort},
    data::Build,
    graph::{NodeIndex, UnGraph},
};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{TokenStreamExt, format_ident, quote};
use std::path::Path;
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

fn mx(bit_sets: &Vec<BitSet>) -> Vec<HashSet<NodeIndex<u32>>> {
    let mut mx = Vec::new();
    let mut set_bit = 0;
    loop {
        let mut result = BitSet::new();
        for bit_set in bit_sets {
            if bit_set.contains(set_bit) {
                result.union_with(bit_set);
            }
        }
        if result.is_empty() {
            break;
        }
        mx.push(result);
        set_bit += 1;
    }
    let mut edges = Vec::new();
    for i in 0..mx.len() {
        for j in (i + 1)..mx.len() {
            if !mx[i].contains(j) && !mx[j].contains(i) {
                edges.push((i, j));
            }
        }
    }
    let mut cliques = Vec::new();
    while edges.len() > 0 {
        let g = UnGraph::<usize, ()>::from_edges(edges.iter().map(|(a, b)| (*a as u32, *b as u32)));
        let c = maximal_cliques(&g)
            .into_iter()
            .max_by_key(|c| c.len())
            .unwrap();
        edges = edges
            .into_iter()
            .filter(|(a, b)| !(c.contains(&NodeIndex::new(*a)) || c.contains(&NodeIndex::new(*b))))
            .collect();
        cliques.push(c);
    }
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
        // let n = tokens.len();
        // let n_lit = LitInt::new(&n.to_string(), Span::call_site());
        let mut bit_sets = Vec::new();
        for tokenised_file_name in &tokenised_file_names {
            let mut bit_set = BitSet::new();
            for t in tokenised_file_name {
                bit_set.insert(token_to_index[t]);
            }
            bit_sets.push(bit_set);
        }
        let mx = mx(&bit_sets);
        let mut mx_enums = Vec::new();
        let enum_name_base = "_MX_";
        for (i, mx) in mx.iter().enumerate() {
            let mx_tokens = mx.iter().map(|index| tokens[index.index()].clone());
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
