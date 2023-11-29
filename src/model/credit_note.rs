
use serde::{Serialize, Deserialize};
use super::{CreditNoteLinesList, CreditNoteTaxAmount, DiscountsResourceDiscountAmount};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreditNote {
    pub amount: i64,
    pub amount_shipping: i64,
    pub created: i64,
    pub currency: String,
    pub customer: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance_transaction: Option<serde_json::Value>,
    pub discount_amount: i64,
    pub discount_amounts: Vec<DiscountsResourceDiscountAmount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<i64>,
    pub id: String,
    pub invoice: serde_json::Value,
    pub lines: CreditNoteLinesList,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub number: String,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of_band_amount: Option<i64>,
    pub pdf: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<serde_json::Value>,
    pub status: String,
    pub subtotal: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subtotal_excluding_tax: Option<i64>,
    pub tax_amounts: Vec<CreditNoteTaxAmount>,
    pub total: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_excluding_tax: Option<i64>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub voided_at: Option<i64>,
}
impl std::fmt::Display for CreditNote {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}