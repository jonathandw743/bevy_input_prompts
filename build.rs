use quote::quote;
use std::fs::{self, DirEntry, File};
use std::io::Write;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let dir_path = "assets/bevy_input_prompts";
    fs::create_dir_all("generated")?;
    let out_path = Path::new("generated/directory_representation.rs");

    let enum_def = directory_representation_module(dir_path);

    // let mut variants = Vec::new();

    // for entry in fs::read_dir(dir_path)? {
    //     let entry = entry?;
    //     let filename = entry
    //         .file_name()
    //         .into_string()
    //         .expect("Could not convert file name to String");

    //     let variant = filename_to_variant(&filename);
    //     variants.push(quote! { #variant, });
    // }

    // let enum_def = quote! {
    //     pub enum Asset {
    //         #(#variants)*
    //     }
    // };

    let mut file = File::create(out_path).unwrap();
    write!(file, "{}", enum_def).unwrap();

    Ok(())
}

fn directory_representation_module<P: AsRef<Path>>(dir: P) -> proc_macro2::TokenStream {
    let mut dirs: Vec<DirEntry> = Vec::new();
    let mut variants = Vec::new();
    for x in fs::read_dir(&dir).unwrap() {
        let x = x.unwrap();
        if x.file_type().unwrap().is_file() {
            let variant = filename_to_variant(x.file_name().to_str().unwrap());
            variants.push(quote! { #variant, });
        }
        if x.file_type().unwrap().is_dir() {
            let variant = filename_to_variant(x.file_name().to_str().unwrap());
            variants.push(quote! { #variant( #variant::Path ), });
            dirs.push(x);
        }
    }
    let mut submodules = Vec::new();
    for x in &dirs {
        submodules.push(directory_representation_module(x.path()))
    }
    let dir_variant = filename_to_variant(dir.as_ref().file_name().unwrap().to_str().unwrap());
    quote! {
        pub mod #dir_variant {
        pub enum Path { #(#variants)* }
            #(#submodules)*
        }
    }
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
