use crate::attributes::DataType;
use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::quote;

impl ToTokens for DataType {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(match self {
            Self::Integer(i) => {
                let optional_len = match i.len {
                    Some(l) => quote! { Some(#l) },
                    None => quote! { None },
                };

                quote! { ColumnType::Integer(#optional_len) }
            },
            Self::String(s) => {
                let optional_len = match s.len {
                    Some(l) => quote! { Some(#l) },
                    None => quote! { None },
                };

                quote! { ColumnType::String(#optional_len) }
            }
        });
    }
}
