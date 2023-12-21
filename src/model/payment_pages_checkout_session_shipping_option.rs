use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentPagesCheckoutSessionShippingOption {
    ///A non-negative integer in cents representing how much to charge.
    pub shipping_amount: i64,
    ///The shipping rate.
    pub shipping_rate: serde_json::Value,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionShippingOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}