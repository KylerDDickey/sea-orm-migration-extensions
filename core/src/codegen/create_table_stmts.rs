use crate::codegen::ColumnDefStmts;
use crate::resource::{ErrorResource, Tokens};
use darling::{Error, FromDeriveInput, ToTokens, FromVariant, Result};
use darling::ast::Data;
use darling::error::Accumulator;
use darling::util::Ignored;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Ident, Variant};

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(FromDeriveInput)]
#[darling(attributes(create_table, col), supports(enum_any))]
pub struct CreateTableStmts {
    pub data: Data<Variant, Ignored>,
    pub ident: Ident,
    #[darling(default)]
    pub if_not_exists: bool,
}

impl CreateTableStmts {
    fn try_get_column_def_token_streams(&self) -> Result<Vec<TokenStream>> {
        let token_streams: Vec<TokenStream> = self.data
            .clone()
            .take_enum()
            .ok_or(Error::custom(ErrorResource::InvalidColApplication))?
            .into_iter()
            .filter_map(|v| {
                if v.ident.eq(&Tokens::Table) {
                    return None;
                }

                let stmts = ColumnDefStmts::from_variant(&v)
                    .map(|c| c.to_tokens_from_partial(&self.ident))
                    .unwrap_or_else(|e| e.write_errors());

                Some(stmts)
            })
            .collect();

        Ok(token_streams)
    }

    fn to_tokens_inner(&self, tokens: &mut TokenStream, err_acc: &mut Accumulator) {
        let table_ident = &self.ident;

        let if_not_exists_ts = match &self.if_not_exists {
            true => quote! { .if_not_exists() },
            false => quote! { },
        };

        let cols = match self.try_get_column_def_token_streams() {
            Ok(token_streams) => token_streams,
            Err(err) => {
                err_acc.push(err);
                Vec::new()
            },
        };

        let up = quote! {
            let mut table = Table::create()
                .table(#table_ident::Table)
                #if_not_exists_ts
                #(.col(#cols))*
                .to_owned();

            manager
                .create_table(table)
                .await
        };

        let down = quote! {
            let table = Table::drop()
                .table(#table_ident::Table)
                .to_owned();

            manager
                .drop_table(table)
                .await
        };

        tokens.extend(super::impl_migration_trait(up, down));
    }
}

impl ToTokens for CreateTableStmts {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let mut err_acc = Error::accumulator();

        self.to_tokens_inner(tokens, &mut err_acc);

        let errors = err_acc
            .into_inner()
            .into_iter()
            .map(Error::write_errors);

        tokens.extend(errors);
    }
}

#[cfg(test)]
mod tests {
    use crate::codegen::CreateTableStmts;
    use darling::{FromDeriveInput, ToTokens};
    use darling::ast::Data;
    use proc_macro2::Span;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{DeriveInput, Fields, Ident, Variant};

    fn generate_migr_code_wrapper(ident_str: &str, if_not_exists: bool) -> TokenStream {
        let create_table = CreateTableStmts {
            ident: Ident::new(ident_str, Span::call_site()),
            data: Data::Enum(vec![
                Variant {
                    ident: Ident::new("Table", Span::call_site()),
                    attrs: Vec::new(),
                    fields: Fields::Unit,
                    discriminant: None,
                },
            ]),
            if_not_exists,
        };

        create_table.to_token_stream()
    }

    #[test]
    fn table_migration_valid_parse_target() {
        let target_table_migr = CreateTableStmts {
            ident: Ident::new("Something", Span::call_site()),
            data: Data::Enum(vec![
                Variant {
                    ident: Ident::new("Table", Span::call_site()),
                    attrs: Vec::new(),
                    fields: Fields::Unit,
                    discriminant: None,
                },
            ]),
            if_not_exists: false,
        };

        let test_mock_parsed_item = syn::parse2::<DeriveInput>(quote! {
            #[create_table]
            enum Something {
                Table,
            }
        });

        let test_table_migr_item_enum = test_mock_parsed_item.unwrap();
        println!("{:?}", test_table_migr_item_enum);
        let test_table_migr = CreateTableStmts::from_derive_input(&test_table_migr_item_enum);

        assert_eq!(target_table_migr, test_table_migr.unwrap());
    }

    #[test]
    fn table_migration_valid_parse_target_ine() {
        let target_table_migr = CreateTableStmts {
            ident: Ident::new("Something", Span::call_site()),
            data: Data::Enum(vec![
                Variant {
                    ident: Ident::new("Table", Span::call_site()),
                    attrs: Vec::new(),
                    fields: Fields::Unit,
                    discriminant: None,
                },
            ]),
            if_not_exists: true,
        };

        let test_mock_parsed_item = syn::parse2::<DeriveInput>(quote! {
            #[create_table(if_not_exists)]
            enum Something {
                Table,
            }
        });

        let test_table_migr_item_enum = test_mock_parsed_item.unwrap();
        println!("{:?}", test_table_migr_item_enum);
        let test_table_migr = CreateTableStmts::from_derive_input(&test_table_migr_item_enum);

        assert_eq!(target_table_migr, test_table_migr.unwrap());
    }

    #[test]
    fn test_migr_code_valid_name_true() {
        let test_migr_code = generate_migr_code_wrapper("Test", true);
        let target_migr_code = quote! {
            #[derive(DeriveMigrationName)]
            pub struct Migration;

            #[async_trait::async_trait]
            impl MigrationTrait for Migration {
                async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                    let mut table = Table::create()
                        .table(Test::Table)
                        .if_not_exists()
                        .to_owned();

                    manager
                        .create_table(table)
                        .await
                }

                async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                    let table = Table::drop()
                        .table(Test::Table)
                        .to_owned();

                    manager
                        .drop_table(table)
                        .await
                }
            }
        };

        assert_eq!(test_migr_code.to_string(), target_migr_code.to_string());
    }

    #[test]
    fn test_migr_code_valid_name_false() {
        let test_migr_code = generate_migr_code_wrapper("Test", false);
        let target_migr_code = quote! {
            #[derive(DeriveMigrationName)]
            pub struct Migration;

            #[async_trait::async_trait]
            impl MigrationTrait for Migration {
                async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                    let mut table = Table::create()
                        .table(Test::Table)
                        .to_owned();

                    manager
                        .create_table(table)
                        .await
                }

                async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
                    let table = Table::drop()
                        .table(Test::Table)
                        .to_owned();

                    manager
                        .drop_table(table)
                        .await
                }
            }
        };

        assert_eq!(test_migr_code.to_string(), target_migr_code.to_string());
    }
}
