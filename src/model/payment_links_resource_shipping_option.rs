use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLinksResourceShippingOption {
    ///A non-negative integer in cents representing how much to charge.
    pub shipping_amount: i64,
    ///The ID of the Shipping Rate to use for this shipping option.
    pub shipping_rate: serde_json::Value,
}
impl std::fmt::Display for PaymentLinksResourceShippingOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}