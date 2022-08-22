use async_trait;
use sea_orm_migration::{DbErr, SchemaManager};

#[async_trait::async_trait]
pub trait MigrationStatementsTrait {
    async fn up(manager: &SchemaManager) -> Result<(), DbErr>;
    async fn down(manager: &SchemaManager) -> Result<(), DbErr>;
}
