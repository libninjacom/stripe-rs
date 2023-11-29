
use serde::{Serialize, Deserialize};
use super::{OneTimePriceData, Period};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceItemPreviewParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoiceitem: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<Period>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<OneTimePriceData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "rust_decimal::serde::str_option")]
    pub unit_amount_decimal: Option<rust_decimal::Decimal>,
}
impl std::fmt::Display for InvoiceItemPreviewParams {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}