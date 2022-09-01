use darling::FromMeta;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Default, FromMeta)]
pub struct String {
    pub len: Option<u32>,
}
