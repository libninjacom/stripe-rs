
use serde::{Serialize, Deserialize};
use super::Item;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentPagesCheckoutSessionListLineItems {
    pub data: Vec<Item>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionListLineItems {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}