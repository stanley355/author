use std::fmt;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum BpsRequestModelEnum {
    Data
}

impl fmt::Display for BpsRequestModelEnum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
