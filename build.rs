use quote::quote;
use std::fs::{self, File};
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
    Ok(if dir.as_ref().is_file() {
        let variant = filename_to_variant(
            dir.as_ref()
                .file_name()
                .unwrap()
                .to_str()
                .expect("Could not convert file_name to_str"),
        );
        let file_name = syn::LitStr::new(
            dir.as_ref()
                .to_str()
                .expect("Could not convert file_name to_str"),
            proc_macro2::Span::call_site(),
        );
        quote! {
            pub const #variant: &'static str = #file_name;
        }
    } else {
        let dir_variant = filename_to_variant(
            dir.as_ref()
                .file_name()
                .expect("Could not get file_name")
                .to_str()
                .expect("Could not convert file_name to_str"),
        );
        let mut submodules = Vec::new();
        for dir_entry in fs::read_dir(&dir)? {
            let dir_entry = dir_entry?;
            submodules.push(directory_representation_module(dir_entry.path())?)
        }
        quote! {pub mod #dir_variant {
            #(#submodules)*
        }}
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
