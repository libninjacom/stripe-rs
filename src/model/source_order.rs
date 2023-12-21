use serde::{Serialize, Deserialize};
use super::{Shipping, SourceOrderItem};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceOrder {
    ///A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount for the order.
    pub amount: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///The email address of the customer placing the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    ///List of items constituting the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<SourceOrderItem>>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,
}
impl std::fmt::Display for SourceOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}