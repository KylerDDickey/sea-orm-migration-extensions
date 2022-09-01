use crate::attributes::DataType;
use darling::FromMeta;
use syn::Lit;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(FromMeta)]
pub struct Col {
    pub auto_increment: bool,
    pub data_type: DataType,
    pub default: Option<Lit>,
    pub nullable: bool,
    pub primary_key: bool,
    pub unique: bool,
}
