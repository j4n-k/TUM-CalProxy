use proc_macro::TokenStream as TokenStream1;
use syn::parse_macro_input;

use crate::common::JsonFileInput;

pub fn course_name_replacer(input: TokenStream1) -> TokenStream1 {
    let JsonFileInput { vis, name, json } = parse_macro_input!(input as JsonFileInput);

    let mut sorted_replacements = json.iter()
        .map(|(replacing, replaced)| (replacing, replaced))
        .collect::<Vec<_>>();

    sorted_replacements.sort_by(|(replacing_a, _), (replacing_b, _)| {
        replacing_b.len().cmp(&replacing_a.len())
    });

    let match_lines = sorted_replacements.into_iter()
        .map(|(replacing, replaced)| {
            quote::quote! {
                name = name.replace(#replacing, #replaced);
            }
        });

    let res = quote::quote! {
        #vis fn #name(mut name: String) -> String {
            #(#match_lines)*
            name
        }
    };

    TokenStream1::from(res)
}
