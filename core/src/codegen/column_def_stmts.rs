use crate::attributes::DataType;
use darling::FromVariant;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Lit};

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(FromVariant)]
#[darling(attributes(col))]
pub struct ColumnDefStmts {
    #[darling(default)]
    pub auto_increment: bool,
    pub data_type: DataType,
    pub default: Option<Lit>,
    pub ident: Ident,
    #[darling(default)]
    pub nullable: bool,
    #[darling(default)]
    pub primary_key: bool,
    #[darling(default)]
    pub unique: bool,
}

impl ColumnDefStmts {
    pub fn to_tokens_from_partial(&self, parent_ident: &Ident) -> TokenStream {
        let ident = &self.ident;

        let data_type = &self.data_type;

        let default = match &self.default {
            Some(lit) => quote! { .default(#lit) },
            None => quote! {},
        };

        let not_null = match &self.nullable {
            true => quote! {},
            false => quote! { .not_null() },
        };

        let unique = match &self.unique {
            true => quote! { .unique_key() },
            false => quote! {},
        };

        let auto_increment = match &self.auto_increment {
            true => quote! { .auto_increment() },
            false => quote! {},
        };

        let primary_key = match &self.primary_key {
            true => quote! { .primary_key() },
            false => quote! {},
        };

        quote! {
            ColumnDef::new_with_type(#parent_ident::#ident, #data_type)
                #default
                #not_null
                #unique
                #auto_increment
                #primary_key
        }
    }
}
