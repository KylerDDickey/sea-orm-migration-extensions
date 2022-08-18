use darling::FromMeta;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Default, FromMeta)]
pub struct Integer {
    pub len: Option<u32>,
}
