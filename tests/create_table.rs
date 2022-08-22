#![allow(dead_code)]

use sea_orm_migration_ex::{
    DeriveMigrationStatementsTrait,
    MigrationStatementsTrait,
};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationStatementsTrait, Iden)]
#[schema_migration(table(create(if_not_exists)))]
enum Post {
    Table,
    #[col(data_type(integer()), auto_increment, primary_key)]
    Id,
    #[col(data_type(string()))]
    Title,
    #[col(data_type(string()))]
    Text,
}

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Post::up(manager).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        Post::down(manager).await?;

        Ok(())
    }
}
