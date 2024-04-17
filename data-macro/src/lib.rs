use proc_macro::TokenStream;

mod building_matcher;
mod common;
mod course_name_replacer;

#[proc_macro]
pub fn building_id_matcher(input: TokenStream) -> TokenStream {
    building_matcher::building_id_matcher(input)
}

#[proc_macro]
pub fn course_name_replacer(input: TokenStream) -> TokenStream {
    course_name_replacer::course_name_replacer(input)
}
