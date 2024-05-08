use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct TopupPayasyougoRequestBody {
    pub user_id: String,
    pub amount: f64,
}
