
use serde::{Serialize, Deserialize};
use super::{LineItemsDiscountAmount, LineItemsTaxAmount};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Item {
    pub amount_discount: i64,
    pub amount_subtotal: i64,
    pub amount_tax: i64,
    pub amount_total: i64,
    pub currency: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<LineItemsDiscountAmount>>,
    pub id: String,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<LineItemsTaxAmount>>,
}
impl std::fmt::Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}