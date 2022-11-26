use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Merchant {
    Duitku,
    Kontenku,
}

impl fmt::Display for Merchant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Merchant::Duitku => write!(f, "Duitku"),
            Merchant::Kontenku => write!(f, "Kontenku"),
        }
    }
}
