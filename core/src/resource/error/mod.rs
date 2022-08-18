use std::fmt::{Display, Formatter, Result};

pub enum ErrorResource {
    InvalidColApplication,
}

impl AsRef<str> for ErrorResource {
    fn as_ref(&self) -> &str {
        match self {
            Self::InvalidColApplication => include_str!("err_invalid_col_application.in"),
        }
    }
}

impl Display for ErrorResource {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(self.as_ref())
    }
}
