use serde::{Serialize, Deserialize};
use super::QuotesResourceUpfront;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuotesResourceComputed {
    ///The definitive totals and line items the customer will be charged on a recurring basis. Takes into account the line items with recurring prices and discounts with `duration=forever` coupons only. Defaults to `null` if no inputted line items with recurring prices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<serde_json::Value>,
    ///
    pub upfront: QuotesResourceUpfront,
}
impl std::fmt::Display for QuotesResourceComputed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}