use proc_macro2::TokenStream;
use quote::quote;
use syn::Ident;

fn impl_migration_statement_tokens(ident: &Ident, up: &TokenStream, down: &TokenStream) -> TokenStream {
    quote! {
        #[async_trait::async_trait]
        impl MigrationStatementTrait for #ident {
            async fn up(manager: &SchemaManager) -> Result<(), DbErr> {
                #up
            }

            async fn down(manager: &SchemaManager) -> Result<(), DbErr> {
                #down
            }
        }
    }
}

pub fn impl_create_table_migration_statement_tokens(ident: &Ident, options: &Vec<TokenStream>, col_stmts: &Vec<TokenStream>) -> TokenStream {
    let up = quote! {
        let table = Table::create()
            .table(Self::Table)
            #(#options)*
            #(.col(#col_stmts))*
            .to_owned();

        manager
            .create_table(table)
            .await
    };

    let down = quote! {
        let table = Table::drop()
            .table(Self::Table)
            .to_owned();

        manager
            .drop_table(table)
            .await
    };

    impl_migration_statement_tokens(ident, &up, &down)
}
