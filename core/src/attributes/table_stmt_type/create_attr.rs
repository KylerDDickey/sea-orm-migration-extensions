use darling::FromMeta;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(Default, FromMeta)]
pub struct Create {
    #[darling(default)]
    pub if_not_exists: bool,
}
