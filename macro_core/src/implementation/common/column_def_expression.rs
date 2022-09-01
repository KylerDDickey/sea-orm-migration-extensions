use crate::attributes::DataType;
use crate::codegen;
use darling::{FromVariant, ToTokens};
use proc_macro2::TokenStream;
use syn::{Ident, Lit};

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(FromVariant)]
#[darling(attributes(col))]
pub struct ColumnDefExpression {
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

impl ToTokens for ColumnDefExpression {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let data_type = match &self.data_type {
            DataType::Integer(i) => codegen::integer_type_tokens(&i.len),
            DataType::String(s) => codegen::string_type_tokens(&s.len),
        };

        let options = vec![
            codegen::default_method_call_tokens(&self.default),
            codegen::not_null_method_call_tokens(!self.nullable),
            codegen::unique_key_method_call_tokens(self.unique),
            codegen::auto_increment_method_call_tokens(self.auto_increment),
            codegen::primary_key_method_call_tokens(self.primary_key),
        ];

        tokens.extend(codegen::column_def_expression_tokens(&self.ident, &data_type, &options));
    }
}
