use proc_macro::TokenStream as TokenStream1;
use syn::parse_macro_input;

use crate::common::JsonFileInput;

pub fn building_id_matcher(input: TokenStream1) -> TokenStream1 {
    let JsonFileInput { vis, name, json } = parse_macro_input!(input as JsonFileInput);

    let match_lines = json.iter().map(|(code, building)| {
        quote::quote! {
            #code => Some(#building)
        }
    });

    let res = quote::quote! {
        #vis fn #name(code: impl AsRef<str>) -> Option<&'static str> {
            match code.as_ref() {
                #(#match_lines,)*
                _ => None
            }
        }
    };

    TokenStream1::from(res)
}
