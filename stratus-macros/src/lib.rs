extern crate proc_macro;
use proc_macro::*;

extern crate syn;

#[proc_macro_attribute]
pub fn timed_fn(_args: TokenStream, input: TokenStream) -> TokenStream {
    use std::str::FromStr;

    let fn_name = {
        let data = input.clone();
        let tokens = syn::parse_macro_input!(data as syn::ItemFn);
        tokens.sig.ident.to_string()
    };

    let mut token_str = input.to_string();
    token_str.insert_str(token_str.find("{").expect("") + 1, 
        &format!("let __internal_stratus_timer = stratus::Timer::new_with_label(\"{}\");", fn_name));
    TokenStream::from_str(&token_str).expect("generated invalid tokens")
}