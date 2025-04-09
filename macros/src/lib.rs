use proc_macro::TokenStream;
use quote::quote;
use std::path::Path;
use syn::{LitStr, parse_macro_input};

#[proc_macro]
pub fn directory_representation(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as LitStr).value();
    let dir_path = Path::new(&input);
    directory_representation_module(
        dir_path,
        dir_path.parent().unwrap_or(Path::new("")),
        &["png", "svg"],
    )
    .expect("Could not create directory representation module")
    .into()
}

fn directory_representation_module<P: AsRef<Path>>(
    dir: P,
    ignore: &Path,
    extension_whitelist: &[&str],
) -> std::io::Result<proc_macro2::TokenStream> {
    Ok(if dir.as_ref().is_file() {
        let Some(ext) = dir.as_ref().extension() else {
            return Ok(quote! {});
        };
        if !extension_whitelist
            .contains(&ext.to_str().expect("Could not convert from OsStr to str"))
        {
            return Ok(quote! {});
        }
        let variant = filename_to_variant(
            dir.as_ref()
                .file_name()
                .unwrap()
                .to_str()
                .expect("Could not convert file name to str"),
        );
        let file_name = syn::LitStr::new(
            dir.as_ref()
                .strip_prefix(ignore)
                .expect("Could not strip prefix")
                .to_str()
                .expect("Could not convert file name to str"),
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
                .expect("Could not convert filename to str"),
        );
        let mut submodules = Vec::new();
        for dir_entry in std::fs::read_dir(&dir)? {
            let dir_entry = dir_entry?;
            submodules.push(directory_representation_module(
                dir_entry.path(),
                ignore,
                extension_whitelist,
            )?)
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
