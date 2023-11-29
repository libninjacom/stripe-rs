
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PromotionCodesResourceRestrictions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<serde_json::Value>,
    pub first_time_transaction: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount_currency: Option<String>,
}
impl std::fmt::Display for PromotionCodesResourceRestrictions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}