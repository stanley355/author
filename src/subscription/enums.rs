use serde::{Deserialize, Serialize};

use std::fmt;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Merchant {
    Midtrans,
    Kontenku,
}

impl fmt::Display for Merchant {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Merchant::Midtrans => write!(f, "Midtrans"),
            Merchant::Kontenku => write!(f, "Kontenku"),
        }
    }
}
