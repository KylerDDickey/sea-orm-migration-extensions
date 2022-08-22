use crate::codegen::util;
use proc_macro2::TokenStream;
use quote::quote;

pub fn if_not_exists_method_call_tokens(flag: bool) -> TokenStream {
    util::flag_tokens(flag, quote! { .if_not_exists() })
}
