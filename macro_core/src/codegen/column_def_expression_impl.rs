use crate::metadata::ColumnDefExpression;
use crate::codegen::util;
use darling::ToTokens;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Lit, Ident};

impl ToTokens for ColumnDefExpression {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let data_type = self.data_type.to_token_stream();

        let options = vec![
            default_method_call_tokens(&self.default),
            not_null_method_call_tokens(!self.nullable),
            unique_key_method_call_tokens(self.unique),
            auto_increment_method_call_tokens(self.auto_increment),
            primary_key_method_call_tokens(self.primary_key),
        ];

        tokens.extend(column_def_expression_tokens(&self.ident, &data_type, &options));
    }
}

fn auto_increment_method_call_tokens(flag: bool) -> TokenStream {
    util::flag_tokens(flag, quote! { .auto_increment() })
}

fn column_def_expression_tokens(ident: &Ident, data_type: &TokenStream, options: &Vec<TokenStream>) -> TokenStream {
    quote! {
        &mut ColumnDef::new_with_type(Self::#ident, #data_type)
            #(#options)*
    }
}

fn default_method_call_tokens(optional_lit: &Option<Lit>) -> TokenStream {
    if let Some(lit) = optional_lit {
        return quote! { .default(#lit) };
    }

    util::blank_tokens()
}

fn not_null_method_call_tokens(flag: bool) -> TokenStream {
    util::flag_tokens(flag, quote! { .not_null() })
}

fn primary_key_method_call_tokens(flag: bool) -> TokenStream {
    util::flag_tokens(flag, quote! { .primary_key() })
}

fn unique_key_method_call_tokens(flag: bool) -> TokenStream {
    util::flag_tokens(flag, quote! { .unique_key() })
}
