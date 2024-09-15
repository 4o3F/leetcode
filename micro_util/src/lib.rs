extern crate proc_macro;
use std::{
    env,
    ffi::OsStr,
    fs,
    path::{Path, PathBuf},
};
mod error;
use crate::error::{Error, Result};
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input, LitStr,
};

struct Arg {
    path: LitStr,
}
impl Parse for Arg {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Arg {
            path: input.parse()?,
        })
    }
}

#[proc_macro]
pub fn gen_clap_handler(dir: TokenStream) -> TokenStream {
    let input = parse_macro_input!(dir as Arg);
    let path = input.path.value();
    let dir = match env::var_os("CARGO_MANIFEST_DIR") {
        Some(manifest_dir) => PathBuf::from(manifest_dir).join(path),
        None => PathBuf::from(path),
    };
    let expanded = match source_file_names(dir) {
        Ok(names) => {
            let names: Vec<_> = names
                .into_iter()
                .map(|name| {
                    let mut module_name = name.replace('-', "_");
                    if module_name.starts_with(|ch: char| ch.is_ascii_digit()) {
                        module_name.insert(0, '_');
                    }
                    let ident = Ident::new(&module_name, Span::call_site());
                    quote! {
                        #ident
                    }
                })
                .collect();

            quote! {
                fn handle(mod_id: &str) {
                    match mod_id {
                        #(
                            stringify!(#names) => problems::#names::run(),
                        )*
                        _ => {
                            eprintln!("Problem not found: {}", mod_id);
                        }
                    }
                }

            }
        }
        Err(err) => syn::Error::new(Span::call_site(), err).to_compile_error(),
    };

    TokenStream::from(expanded)
}

fn source_file_names<P: AsRef<Path>>(dir: P) -> Result<Vec<String>> {
    let mut names = Vec::new();
    let mut failures = Vec::new();

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        if !entry.file_type()?.is_file() {
            continue;
        }

        let file_name = entry.file_name();
        if file_name == "mod.rs" || file_name == "lib.rs" || file_name == "main.rs" {
            continue;
        }

        let path = Path::new(&file_name);
        if path.extension() == Some(OsStr::new("rs")) {
            match file_name.into_string() {
                Ok(mut utf8) => {
                    utf8.truncate(utf8.len() - ".rs".len());
                    names.push(utf8);
                }
                Err(non_utf8) => {
                    failures.push(non_utf8);
                }
            }
        }
    }

    failures.sort();
    if let Some(failure) = failures.into_iter().next() {
        return Err(Error::Utf8(failure));
    }

    if names.is_empty() {
        return Err(Error::Empty);
    }

    names.sort();
    Ok(names)
}
