use proc_macro::TokenStream;
use sea_orm_migration_ex_core::{Derive, DeriveMigrationTrait};
use syn::parse_macro_input;

#[proc_macro_derive(DeriveMigrationTrait, attributes(create_table, col))]
pub fn derive_migration_trait(item: TokenStream) -> TokenStream {
    DeriveMigrationTrait::derive(&parse_macro_input!(item)).into()
}
