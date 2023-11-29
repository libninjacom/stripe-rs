
use serde::{Serialize, Deserialize};
use super::{CreditNoteTaxAmount, DiscountsResourceDiscountAmount, TaxRate};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditNoteLineItem {
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_excluding_tax: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub discount_amount: i64,
    pub discount_amounts: Vec<DiscountsResourceDiscountAmount>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<String>,
    pub livemode: bool,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    pub tax_amounts: Vec<CreditNoteTaxAmount>,
    pub tax_rates: Vec<TaxRate>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "rust_decimal::serde::str")]
    pub unit_amount_decimal: Option<rust_decimal::Decimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "rust_decimal::serde::str")]
    pub unit_amount_excluding_tax: Option<rust_decimal::Decimal>,
}
impl std::fmt::Display for CreditNoteLineItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}