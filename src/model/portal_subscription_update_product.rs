use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortalSubscriptionUpdateProduct {
    ///The list of price IDs which, when subscribed to, a subscription can be updated.
    pub prices: Vec<String>,
    ///The product ID.
    pub product: String,
}
impl std::fmt::Display for PortalSubscriptionUpdateProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}