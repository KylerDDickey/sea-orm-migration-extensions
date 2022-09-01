mod create_attr;

pub use create_attr::Create;
use darling::FromMeta;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(FromMeta)]
pub enum TableStatementType {
    Create(Create),
}
