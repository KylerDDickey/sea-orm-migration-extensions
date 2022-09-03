use crate::emitters::common::emit_derive;
use crate::metadata::TableMigrationStatement;
use proc_macro2::TokenStream;
use syn::DeriveInput;

pub fn derive_migration_statement(item: &DeriveInput) -> TokenStream {
    emit_derive::<TableMigrationStatement>(item)
}
