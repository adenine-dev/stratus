extern crate proc_macro;
use proc_macro::*;

//TODO: make this work with the function name
#[proc_macro_attribute]
pub fn timed_fn(args: TokenStream, input: TokenStream) -> TokenStream {
    use std::str::FromStr;

    let mut token_str = input.to_string();
    token_str.insert_str(token_str.find("{").expect("") + 1, "let __internal_stratus_timer = stratus::Timer::default();");
    TokenStream::from_str(&token_str).expect("generated invalid tokens")
}