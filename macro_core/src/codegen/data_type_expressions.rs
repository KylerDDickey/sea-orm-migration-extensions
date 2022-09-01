use crate::codegen::util::optional_tokens;
use proc_macro2::TokenStream;
use quote::quote;

pub fn string_type_tokens(optional_len: &Option<u32>) -> TokenStream {
    let optional_len_ts = optional_tokens(optional_len);
    quote! { ColumnType::String(#optional_len_ts) }
}

pub fn integer_type_tokens(optional_len: &Option<u32>) -> TokenStream {
    let optional_len_ts = optional_tokens(optional_len);
    quote! { ColumnType::Integer(#optional_len_ts) }
}
