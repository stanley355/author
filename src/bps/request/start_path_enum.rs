use std::fmt;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub enum BpsRequestStartPathEnum {
    List,
    View,
    Dataexim
}

impl fmt::Display for BpsRequestStartPathEnum{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
