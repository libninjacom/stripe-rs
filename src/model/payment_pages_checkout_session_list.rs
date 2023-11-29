
use serde::{Serialize, Deserialize};
use super::CheckoutSession;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentPagesCheckoutSessionList {
    pub data: Vec<CheckoutSession>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}