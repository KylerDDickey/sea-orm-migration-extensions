use proc_macro::TokenStream;
use sea_orm_migration_ex_macro_core::derive_migration_statement;
use syn::parse_macro_input;

#[proc_macro_derive(
    MigrationStatementTrait,
    attributes(schema_migration, col))]
pub fn derive_migration_statement_trait(item: TokenStream) -> TokenStream {
    derive_migration_statement(&parse_macro_input!(item)).into()
}
