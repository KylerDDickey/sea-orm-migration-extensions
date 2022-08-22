use proc_macro::TokenStream;
use sea_orm_migration_ex_core::{Derive, DeriveMigrationStatementsTrait};
use syn::parse_macro_input;

#[proc_macro_derive(
    DeriveMigrationStatementsTrait,
    attributes(schema_migration, col))]
pub fn derive_migration_statements_trait(item: TokenStream) -> TokenStream {
    DeriveMigrationStatementsTrait::derive(&parse_macro_input!(item)).into()
}
