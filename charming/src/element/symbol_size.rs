use serde::Serialize;

use super::RawString;

#[derive(Serialize)]
#[serde(untagged)]
pub enum SymbolSize {
    Number(i64),
    Function(RawString),
}

impl From<i64> for SymbolSize {
    fn from(n: i64) -> Self {
        SymbolSize::Number(n as i64)
    }
}


impl From<&str> for SymbolSize {
    fn from(s: &str) -> Self {
        SymbolSize::Function(RawString::from(s))
    }
}
