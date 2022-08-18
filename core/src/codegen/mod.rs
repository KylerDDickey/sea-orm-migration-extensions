mod column_def_stmts;
mod create_table_stmts;
mod data_type_stmts;

pub use column_def_stmts::ColumnDefStmts;
pub use create_table_stmts::CreateTableStmts;

use proc_macro2::TokenStream;
use quote::quote;

/// Generates the standard sea-orm migration implementation pattern given
/// the migrate up and migrate down logic.
fn impl_migration_trait(up: TokenStream, down: TokenStream)  -> TokenStream {
    quote! {
        #[derive(DeriveMigrationName)]
        pub struct Migration;

        #[async_trait::async_trait]
        impl MigrationTrait for Migration {
            async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                #up
            }

            async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                #down
            }
        }
    }
}
