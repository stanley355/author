use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct StudentDiscountAvailabilityRes {
  pub is_student: bool,
  pub is_free_discount: bool,
  pub is_half_discount: bool
}

impl StudentDiscountAvailabilityRes {
    pub fn to_tuple(self) -> (bool, bool, bool) {
      (self.is_student, self.is_free_discount, self.is_half_discount)
    }
}