use crate::attributes::TableStatementType;
use darling::FromDeriveInput;
use darling::ast::Data;
use darling::util::Ignored;
use syn::{Ident, Variant};

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(FromDeriveInput)]
#[darling(attributes(schema_migration), supports(enum_any))]
pub struct TableMigrationStatement {
    pub data: Data<Variant, Ignored>,
    pub ident: Ident,
    #[darling(rename = "table")]
    pub table_statement_type: TableStatementType,
}
