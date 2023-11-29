
use serde::{Serialize, Deserialize};
use super::QuotesResourceTotalDetailsResourceBreakdown;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuotesResourceTotalDetails {
    pub amount_discount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_shipping: Option<i64>,
    pub amount_tax: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<QuotesResourceTotalDetailsResourceBreakdown>,
}
impl std::fmt::Display for QuotesResourceTotalDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}