#![allow(dead_code)]

use sea_orm_migration_ex::DeriveMigrationTrait;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationTrait, Iden)]
#[create_table(if_not_exists)]
enum Post {
    Table,
    #[col(data_type(integer()), auto_increment, primary_key)]
    Id,
    #[col(data_type(string()))]
    Title,
    #[col(data_type(string()))]
    Text,
}
