use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DokuCheckoutPaymentReq {
  order: DokuCheckoutPaymentOrderReq,
  payment: DokuCheckoutPaymentPaymentReq,
  customer: DokuCheckoutPaymentCustomerReq
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DokuCheckoutPaymentOrderReq {
  amount: u32,
  invoice_number: String,
  callback_url: String
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DokuCheckoutPaymentPaymentReq {
  payment_due_date: u32
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DokuCheckoutPaymentCustomerReq {
  name: String,
  email: String
}



