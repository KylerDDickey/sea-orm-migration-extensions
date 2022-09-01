use darling::{Error, FromDeriveInput, ToTokens};
use proc_macro2::TokenStream;
use syn::DeriveInput;

pub fn emit_derive<T: ToTokens + FromDeriveInput>(item: &DeriveInput) -> TokenStream {
    T::from_derive_input(item)
        .map(T::into_token_stream)
        .unwrap_or_else(Error::write_errors)
}
