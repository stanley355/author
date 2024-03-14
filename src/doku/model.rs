use serde::{Deserialize, Serialize};
use actix_web::web;

use crate::{db::PgPool, topup::model::TopUp};
use super::req::DokuCheckoutPaymentReq;

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Doku;

impl Doku {
    pub fn new_checkout_payment(pool: &web::Data<PgPool>, topup: &TopUp) {
      let req_body = DokuCheckoutPaymentReq::new(pool, topup);

      ()
    }
}

