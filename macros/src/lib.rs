use anyhow::{Result, anyhow};
use petgraph::{Graph, algo::toposort, data::Build};
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use std::{collections::HashMap, path::Path};
use syn::{
    Expr, LitStr, Token,
    parse::{self, Parse},
    parse_macro_input, parse2,
    punctuated::Punctuated,
    token::{Comma, Struct},
};

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
        let mut str_to_i = HashMap::new();
        let mut g = Graph::<String, ()>::new();
        let mut submodules = Vec::new();
        for dir_entry in std::fs::read_dir(&dir)? {
            let path = dir_entry?.path();
            if path.is_file() {
                let tokens = path
                    .file_stem()
                    .ok_or(anyhow!("Could not get file stem"))?
                    .to_str()
                    .ok_or(anyhow!("Could not convert file stem to str"))?
                    .split("_")
                    .collect::<Vec<_>>();
                let mut nis = Vec::new();
                for token in tokens {
                    nis.push(
                        str_to_i
                            .entry(token.to_string())
                            .or_insert_with(|| g.add_node(token.to_string()))
                            .clone(),
                    );
                }
                for i in 0..(nis.len() - 1) {
                    g.add_edge(nis[i], nis[i + 1], ());
                }
            }
            if path.is_dir() {
                submodules.push(directory_representation_module(path, ignore)?)
            }
        }
        let mut variants = Vec::new();
        let mut arms = Vec::new();
        for ni in toposort(&g, None).map_err(|_| anyhow!("Could not topologically sort {:?}", dir.as_ref()))? {
            let token = g
                .node_weight(ni)
                .ok_or(anyhow!("Could not map node index to token"))?;
            let variant = filename_to_variant(&token);
            let token_lit = syn::LitStr::new(&token, Span::call_site());
            variants.push(variant.clone());
            arms.push(quote! {
                Self::#variant => #token_lit
            });
        }
        return Ok(quote! {pub mod #dir_variant {
            pub const PATH: &'static str = #file_name;
            pub enum Tokens {
                #(#variants,)*
            }
            impl Tokens {
                pub fn str(&self) -> &'static str {
                    match self {
                        #(#arms,)*
                    }
                }
            }
            #(#submodules)*
        }});
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
