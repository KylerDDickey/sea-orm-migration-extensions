pub enum Tokens {
    Table,
}

impl AsRef<str> for Tokens {
    fn as_ref(&self) -> &str {
        match self {
            Self::Table => "Table",
        }
    }
}
