use crate::attributes::{Create, TableStatementType};
use crate::codegen;
use crate::implementation::ColumnDefExpression;
use crate::resource::{ErrorResource, Tokens};
use darling::{Error, FromDeriveInput, FromVariant, Result, ToTokens};
use darling::ast::Data;
use darling::error::Accumulator;
use darling::util::Ignored;
use proc_macro2::TokenStream;
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

impl TableMigrationStatement {
    fn handle_create_stmt(&self, create: &Create, variants: &Vec<Variant>, err_acc: &mut Accumulator) -> TokenStream {
        let options = vec![
            codegen::if_not_exists_method_call_tokens(create.if_not_exists),
        ];

        let col_stmts = variants
            .into_iter()
            .filter_map(|v| {
                match ColumnDefExpression::from_variant(&v) {
                    Ok(col_stmts) => Some(ColumnDefExpression::into_token_stream(col_stmts)),
                    Err(err) => {
                        err_acc.push(err);
                        None
                    },
                }
            })
            .collect::<Vec<TokenStream>>();

        codegen::impl_create_table_migration_statement_tokens(&self.ident, &options, &col_stmts)
    }

    fn to_tokens_inner(&self, tokens: &mut TokenStream, err_acc: &mut Accumulator) {
        let variants = match self.try_get_column_variants() {
            Ok(token_streams) => token_streams,
            Err(err) => {
                err_acc.push(err);
                Vec::new()
            },
        };

        tokens.extend(match &self.table_statement_type {
            TableStatementType::Create(c) => self.handle_create_stmt(&c, &variants, err_acc),
        });
    }

    fn try_get_column_variants(&self) -> Result<Vec<Variant>> {
        let token_streams = self.data
            .clone()
            .take_enum()
            .ok_or(Error::custom(ErrorResource::InvalidColApplication))?
            .into_iter()
            .filter(|v| v.ident.ne(&Tokens::Table))
            .collect();

        Ok(token_streams)
    }
}

impl ToTokens for TableMigrationStatement {
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
    use crate::attributes::{Create, TableStatementType};
    use crate::implementation::TableMigrationStatement;
    use darling::{FromDeriveInput, ToTokens};
    use darling::ast::Data;
    use proc_macro2::Span;
    use proc_macro2::TokenStream;
    use quote::quote;
    use syn::{DeriveInput, Fields, Ident, Variant};

    fn generate_migr_code_wrapper(ident_name: &str, if_not_exists: bool) -> TokenStream {
        let create_table = TableMigrationStatement {
            data: Data::Enum(vec![
                Variant {
                    ident: Ident::new("Table", Span::call_site()),
                    attrs: Vec::new(),
                    fields: Fields::Unit,
                    discriminant: None,
                },
            ]),
            ident: Ident::new(ident_name, Span::call_site()),
            table_statement_type: TableStatementType::Create(Create {
                if_not_exists,
            }),
        };

        create_table.to_token_stream()
    }

    #[test]
    fn table_migration_valid_parse_target() {
        let target_table_migr = TableMigrationStatement {
            data: Data::Enum(vec![
                Variant {
                    ident: Ident::new("Table", Span::call_site()),
                    attrs: Vec::new(),
                    fields: Fields::Unit,
                    discriminant: None,
                },
            ]),
            ident: Ident::new("Something", Span::call_site()),
            table_statement_type: TableStatementType::Create(Create {
                if_not_exists: false,
            }),
        };

        let test_mock_parsed_item = syn::parse2::<DeriveInput>(quote! {
            #[schema_migration(table(create()))]
            enum Something {
                Table,
            }
        });

        let test_table_migr_item_enum = test_mock_parsed_item.unwrap();
        let test_table_migr = TableMigrationStatement::from_derive_input(&test_table_migr_item_enum);

        assert_eq!(target_table_migr, test_table_migr.unwrap());
    }

    #[test]
    fn table_migration_valid_parse_target_ine() {
        let target_table_migr = TableMigrationStatement {
            data: Data::Enum(vec![
                Variant {
                    ident: Ident::new("Table", Span::call_site()),
                    attrs: Vec::new(),
                    fields: Fields::Unit,
                    discriminant: None,
                },
            ]),
            ident: Ident::new("Something", Span::call_site()),
            table_statement_type: TableStatementType::Create(Create {
                if_not_exists: true,
            }),
        };

        let test_mock_parsed_item = syn::parse2::<DeriveInput>(quote! {
            #[schema_migration(table(create(if_not_exists)))]
            enum Something {
                Table,
            }
        });

        let test_table_migr_item_enum = test_mock_parsed_item.unwrap();
        let test_table_migr = TableMigrationStatement::from_derive_input(&test_table_migr_item_enum);

        assert_eq!(target_table_migr, test_table_migr.unwrap());
    }

    #[test]
    fn test_migr_code_valid_name_true() {
        let test_migr_code = generate_migr_code_wrapper("Test", true);
        let target_migr_code = quote! {
            #[async_trait::async_trait]
            impl MigrationStatementTrait for Test {
                async fn up(manager: &SchemaManager) -> Result<(), DbErr> {
                    let table = Table::create()
                        .table(Self::Table)
                        .if_not_exists()
                        .to_owned();

                    manager
                        .create_table(table)
                        .await
                }

                async fn down(manager: &SchemaManager) -> Result<(), DbErr> {
                    let table = Table::drop()
                        .table(Self::Table)
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
            #[async_trait::async_trait]
            impl MigrationStatementTrait for Test {
                async fn up(manager: &SchemaManager) -> Result<(), DbErr> {
                    let table = Table::create()
                        .table(Self::Table)
                        .to_owned();

                    manager
                        .create_table(table)
                        .await
                }

                async fn down(manager: &SchemaManager) -> Result<(), DbErr> {
                    let table = Table::drop()
                        .table(Self::Table)
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
