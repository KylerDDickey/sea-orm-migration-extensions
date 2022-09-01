use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::quote;

pub fn blank_tokens() -> TokenStream {
    quote! { }
}

pub fn flag_tokens(flag: bool, ts: TokenStream) -> TokenStream {
    if flag {
        return ts;
    }

    blank_tokens()
}

pub fn optional_tokens<T: ToTokens>(optional: &Option<T>) -> TokenStream {
    if let Some(x) = optional {
        return quote! { Some(#x) };
    }

    quote! { None }
}
