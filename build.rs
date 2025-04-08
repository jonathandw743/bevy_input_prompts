use quote::quote;
use std::fs::{self, DirEntry, File};
use std::io::Write;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let dir_path = "assets/bevy_input_prompts";
    let out_path = Path::new("generated/directory_representation.rs");
    if let Some(parent) = out_path.parent() {
        fs::create_dir_all(parent)?;
    }
    let mut file = File::create(out_path)?;
    write!(file, "{}", directory_representation_module(dir_path)?).unwrap();
    Ok(())
}

fn directory_representation_module<P: AsRef<Path>>(
    dir: P,
) -> std::io::Result<proc_macro2::TokenStream> {
    let mut dirs: Vec<DirEntry> = Vec::new();
    let mut variants = Vec::new();
    let mut branches = Vec::new();
    for dir_entry in fs::read_dir(&dir)? {
        let dir_entry = dir_entry?;
        if dir_entry.file_type().unwrap().is_file() {
            let variant = filename_to_variant(
                dir_entry
                    .file_name()
                    .to_str()
                    .expect("Could not convert file_name to_str"),
            );
            let file_name = syn::LitStr::new(
                dir_entry
                    .path()
                    .to_str()
                    .expect("Could not convert file_name to_str"),
                proc_macro2::Span::call_site(),
            );
            variants.push(quote! { #variant, });
            branches.push(quote! { Self::#variant => #file_name, });
        }
        if dir_entry.file_type().unwrap().is_dir() {
            let variant = filename_to_variant(
                dir_entry
                    .file_name()
                    .to_str()
                    .expect("Could not convert file_name to_str"),
            );
            variants.push(quote! { #variant( #variant::Path ), });
            dirs.push(dir_entry);
        }
    }
    let mut submodules = Vec::new();
    for x in &dirs {
        submodules.push(directory_representation_module(x.path())?)
    }
    let dir_variant = filename_to_variant(
        dir.as_ref()
            .file_name()
            .expect("Could not get file_name")
            .to_str()
            .expect("Could not convert file_name to_str"),
    );
    let impl_block = if variants.len() == branches.len() {
        Some(quote! {
            pub enum Path { #(#variants)* }
            impl Path {
                pub fn path(&self) -> std::path::Path {
                    match self {
                        #(#branches)*
                    }
                }
            }
        })
    } else {
        None
    };
    Ok(quote! {
        pub mod #dir_variant {
            #impl_block
            #(#submodules)*
        }
    })
}

fn filename_to_variant(name: &str) -> proc_macro2::Ident {
    let base = name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect::<String>();

    let base = if base
        .chars()
        .next()
        .expect("Could not get first character of variant")
        .is_numeric()
    {
        format!("_{}", base)
    } else {
        base
    };

    syn::parse_str::<proc_macro2::Ident>(&base).unwrap()
}
