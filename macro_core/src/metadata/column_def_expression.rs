use crate::attributes::DataType;
use darling::FromVariant;
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
