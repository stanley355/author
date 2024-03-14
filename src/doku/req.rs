use actix_web::web;
use serde::{Deserialize, Serialize};

use crate::{db::PgPool, topup::model::TopUp, user::model::User};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DokuCheckoutPaymentReq {
    order: DokuCheckoutPaymentOrderReq,
    payment: DokuCheckoutPaymentPaymentReq,
    customer: DokuCheckoutPaymentCustomerReq,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DokuCheckoutPaymentOrderReq {
    amount: u32,
    invoice_number: String,
    callback_url: String,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DokuCheckoutPaymentPaymentReq {
    payment_due_date: u32,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct DokuCheckoutPaymentCustomerReq {
    name: String,
    email: String,
}

impl DokuCheckoutPaymentReq {
    pub fn new(pool: &web::Data<PgPool>, topup: &TopUp) -> Self {
        let user_result = User::find_by_id(pool, &topup.user_id.to_string());
        let req = DokuCheckoutPaymentReq {
            order: DokuCheckoutPaymentOrderReq {
                amount: topup.topup_amount as u32,
                invoice_number: topup.id.to_string(),
                callback_url: "woi".to_string(),
            },
            payment: DokuCheckoutPaymentPaymentReq {
                payment_due_date: 60,
            },
            customer: match user_result {
                Ok(user) => DokuCheckoutPaymentCustomerReq {
                    name: user.fullname,
                    email: user.email,
                },
                Err(_) => DokuCheckoutPaymentCustomerReq {
                    name: "".to_string(),
                    email: "".to_string(),
                },
            },
        };

        return req;
    }
}
