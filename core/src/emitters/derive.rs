use darling::{Error, FromDeriveInput, ToTokens};
use proc_macro2::TokenStream;
use syn::DeriveInput;

pub trait Derive<T: ToTokens + FromDeriveInput> {
    fn derive(item: &DeriveInput) -> TokenStream {
        T::from_derive_input(item)
            .map(T::into_token_stream)
            .unwrap_or_else(Error::write_errors)
    }
}
