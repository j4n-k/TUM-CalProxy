use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use syn::parse::{Parse, ParseStream};
use syn::{parenthesized, Error, Visibility};

pub(crate) struct JsonFileInput {
    pub(crate) vis: Visibility,
    pub(crate) name: syn::Ident,
    pub(crate) json: HashMap<String, String>,
}

impl Parse for JsonFileInput {
    fn parse(input: ParseStream) -> Result<Self, Error> {
        let vis = input.parse::<Visibility>()?;
        let _ = input.parse::<syn::Token![fn]>()?;
        let name = input.parse::<syn::Ident>()?;

        let content;
        let _ = parenthesized!(content in input);

        let path = content.parse::<syn::LitStr>()?;
        let span = path.span();

        let Ok(path) = path.value().parse::<PathBuf>() else {
            unreachable!("For some reason rustc thinks parse can return an error here")
        };

        let file = match File::open(&path) {
            Ok(file) => file,
            Err(err) => return Err(Error::new(span, format!("Invalid path {}", err))),
        };

        let json = match serde_json::from_reader(file) {
            Ok(json) => json,
            Err(err) => return Err(Error::new(span, format!("Invalid JSON: {}", err))),
        };

        Ok(Self { vis, name, json })
    }
}
