use serde::{Serialize, Deserialize};
use super::TaxRate;
///An Add Invoice Item describes the prices and quantities that will be added as pending invoice items when entering a phase.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionScheduleAddInvoiceItem {
    ///ID of the price used to generate the invoice item.
    pub price: serde_json::Value,
    ///The quantity of the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<i64>,
    ///The tax rates which apply to the item. When set, the `default_tax_rates` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<TaxRate>>,
}
impl std::fmt::Display for SubscriptionScheduleAddInvoiceItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}