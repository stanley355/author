use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
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
