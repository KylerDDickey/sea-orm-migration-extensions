mod integer;
mod string;

use darling::FromMeta;
pub use integer::Integer;
pub use string::String;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(FromMeta)]
pub enum DataType {
    Integer(Integer),
    String(String),
}
