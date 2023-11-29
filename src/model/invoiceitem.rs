
use serde::{Serialize, Deserialize};
use super::{InvoiceLineItemPeriod, TaxRate};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Invoiceitem {
    pub amount: i64,
    pub currency: String,
    pub customer: serde_json::Value,
    pub date: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub discountable: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<serde_json::Value>>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<serde_json::Value>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub object: String,
    pub period: InvoiceLineItemPeriod,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<serde_json::Value>,
    pub proration: bool,
    pub quantity: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_item: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<TaxRate>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "rust_decimal::serde::str_option")]
    pub unit_amount_decimal: Option<rust_decimal::Decimal>,
}
impl std::fmt::Display for Invoiceitem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}