use serde::{Serialize, Deserialize};
use super::RecurringAdhoc;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecurringPriceData {
    pub currency: String,
    pub product: String,
    pub recurring: RecurringAdhoc,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "rust_decimal::serde::str_option")]
    pub unit_amount_decimal: Option<rust_decimal::Decimal>,
}
impl std::fmt::Display for RecurringPriceData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}