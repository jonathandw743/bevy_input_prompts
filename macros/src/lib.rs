use anyhow::{Result, anyhow};
use fixedbitset::FixedBitSet;
use hashbrown::HashMap;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use std::path::{Path, PathBuf};
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
        .to_token_stream(
            
        )
        .expect("Could not create directory representation module")
        .into()
}

fn tokenise_dir_entry(path: &PathBuf) -> Result<(Vec<String>, String)> {
    Ok((
        path.file_stem()
            .ok_or(anyhow!("Could not get file stem"))?
            .to_str()
            .ok_or(anyhow!("Could not convert file stem to str"))?
            .split("_")
            .map(|x| x.to_owned())
            .collect::<Vec<_>>(),
        path.to_str()
            .ok_or(anyhow!("Could not convert file path to str"))?
            .to_owned(),
    ))
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

struct DirectoryRepresentationIntermediary {
    dir_variant: Ident,
    file_name: LitStr,
    token_sets_file_paths: Vec<(Vec<String>, String)>,
    sub_dirs: Vec<PathBuf>,
    tokens: Vec<String>,
    token_to_index: HashMap<String, usize>,
    num_files: usize,
    num_tokens: usize,
    bit_sets: Vec<FixedBitSet>,
    non_exclusive: Vec<FixedBitSet>,
    mx_count: usize,
    coloring: Vec<usize>,
    graph_coloring: fn(&Vec<FixedBitSet>, usize) -> (usize, Vec<usize>),
}

impl DirectoryRepresentationIntermediary {
    fn from_path<P: AsRef<Path>>(dir: P, graph_coloring: fn(&Vec<FixedBitSet>, usize) -> (usize, Vec<usize>)) -> Result<Self> {
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
        let mut token_sets_file_paths = Vec::new();
        // modules corresponding to subdirectories
        let mut sub_dirs = Vec::new();
        for dir_entry in std::fs::read_dir(&dir)? {
            let path = dir_entry?.path();
            if path.is_file() {
                token_sets_file_paths.push(tokenise_dir_entry(&path)?);
            }
            if path.is_dir() {
                sub_dirs.push(path)
            }
        }
        // make unique (not using a hash set because i want a deterministic iteration)
        let mut tokens: Vec<_> = token_sets_file_paths
        .iter()
        .map(|(tokens, _)| tokens.clone())
            .flatten()
            .collect();
        tokens.sort();
        tokens.dedup();
        // create a map from tokens to indices in tokens
        let mut token_to_index = HashMap::new();
        for (i, token) in tokens.iter().enumerate() {
            token_to_index.insert(token.clone(), i);
        }
        
        let num_files = token_sets_file_paths.len();
        let num_tokens = tokens.len();
        // convert each file into a bit set that says whether the file name contains a given token
        // assumes there aren't file names that are made of the same set of tokens e.g. keyboard_w and w_keyboard
        let mut bit_sets = vec![FixedBitSet::with_capacity(num_tokens); num_files];
        for (i, (token_set, file_name)) in token_sets_file_paths.iter().enumerate() {
            for token in token_set {
                bit_sets[i].insert(token_to_index[token]);
            }
        }
        let non_exclusive = non_exclusive(&bit_sets, num_tokens);
        
        let (mx_count, coloring) = graph_coloring(&non_exclusive, num_tokens);

        let min_colors = token_sets_file_paths.iter().map(|(a, _)| a.len()).max().unwrap_or(0);
        if min_colors != mx_count {
            dbg!(min_colors, mx_count, dir.as_ref());
        }

        Ok(Self {
            dir_variant,
            file_name,
            token_sets_file_paths,
            sub_dirs,
            tokens,
            token_to_index,
            num_files,
            num_tokens,
            bit_sets,
            non_exclusive,
            mx_count,
            coloring,
            graph_coloring,
        })
    }

    fn to_token_stream(&self) -> Result<proc_macro2::TokenStream> {

        let mx_enum_names: Vec<_> = (0..self.mx_count)
            .map(|color| format_ident!("_MX_{}", color))
            .collect();
        let mut mx_enum_variants = vec![Vec::new();self.mx_count];
        for (&color, token) in self.coloring.iter().zip(self.tokens.iter()) {
            mx_enum_variants[color].push(filename_to_variant(&token));
        }
        let mx_enums = mx_enum_names
            .iter()
            .zip(mx_enum_variants)
            .map(|(enum_name, variants)| {
                quote! {
                    pub enum #enum_name {
                        #(#variants,)*
                    }
                }
            });
        
        let mut function_arms = Vec::with_capacity(self.num_files);
        for (token_set, file_name) in self.token_sets_file_paths.iter() {
            let mut variant_exprs = vec![None; self.mx_count];
            for token in token_set {
                let color = self.coloring[self.token_to_index[token]];
                let enum_name = &mx_enum_names[color];
                let variant = filename_to_variant(&token);
                variant_exprs[color] = Some(quote! { #enum_name::#variant });
            }
            let variant_exprs_unwrap = variant_exprs.into_iter().map(|v| match v {
                Some(v) => quote! { Some(#v) },
                None => quote! { None },
            });
            let lit = LitStr::new(file_name, Span::call_site());
            function_arms.push(quote! { (#(#variant_exprs_unwrap,)*) => Some(#lit) });
        }
        let tctype = (0..self.mx_count).map(|i| {
            let e = format_ident!("_MX_{}", i);
            quote! {
                Option<#e>
            }
        });
        let function = 
        // if self.num_files == 0 {
        //     quote! {}
        // } else {
            quote! {
                pub fn file(tc: (#(#tctype,)*)) -> Option<&'static str> {
                    match tc {
                        #(#function_arms,)*
                        _ => None
                    }
                }
            }
        // }
        ;
        let mut submodules = Vec::new();
        for sub_dir in &self.sub_dirs {
            submodules.push(DirectoryRepresentationIntermediary::from_path(sub_dir, self.graph_coloring)?.to_token_stream()?);
        }

        let dir_variant = &self.dir_variant;
        let file_name  = &self.file_name;

        Ok(quote! {
            pub mod #dir_variant {
                pub const PATH: &'static str = #file_name;
                #(#mx_enums)*
                #function
                #(#submodules)*
            }
        })
    }
}

fn filename_to_variant(name: &str) -> proc_macro2::Ident {
    let base = name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>();
    let base = format!("_{}", base);
    syn::parse_str::<proc_macro2::Ident>(&base).unwrap()
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