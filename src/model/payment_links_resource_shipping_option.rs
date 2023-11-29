
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceShippingOption {
    pub shipping_amount: i64,
    pub shipping_rate: serde_json::Value,
}
impl std::fmt::Display for PaymentLinksResourceShippingOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}