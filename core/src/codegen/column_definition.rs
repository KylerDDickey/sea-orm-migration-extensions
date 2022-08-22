use crate::codegen::util;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Lit, Ident};

pub fn auto_increment_method_call_tokens(flag: bool) -> TokenStream {
    util::flag_tokens(flag, quote! { .auto_increment() })
}

pub fn column_def_expression_tokens(ident: &Ident, data_type: &TokenStream, options: &Vec<TokenStream>) -> TokenStream {
    quote! {
        &mut ColumnDef::new_with_type(Self::#ident, #data_type)
            #(#options)*
    }
}

pub fn default_method_call_tokens(optional_lit: &Option<Lit>) -> TokenStream {
    if let Some(lit) = optional_lit {
        return quote! { .default(#lit) };
    }

    util::blank_tokens()
}

pub fn not_null_method_call_tokens(flag: bool) -> TokenStream {
    util::flag_tokens(flag, quote! { .not_null() })
}

pub fn primary_key_method_call_tokens(flag: bool) -> TokenStream {
    util::flag_tokens(flag, quote! { .primary_key() })
}

pub fn unique_key_method_call_tokens(flag: bool) -> TokenStream {
    util::flag_tokens(flag, quote! { .unique_key() })
}
