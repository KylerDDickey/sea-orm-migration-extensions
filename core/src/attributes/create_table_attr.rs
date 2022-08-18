use darling::FromMeta;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Default, FromMeta)]
pub struct CreateTable {
    pub if_not_exists: bool,
}
