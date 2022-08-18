mod integer;
mod string;

use darling::FromMeta;
use integer::Integer;
use string::String;

#[cfg_attr(test, derive(Debug, PartialEq))]
#[derive(FromMeta)]
pub enum DataType {
    Integer(Integer),
    String(String),
}
