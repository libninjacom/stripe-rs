
use serde::{Serialize, Deserialize};
use super::{Shipping, SourceOrderItem};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceOrder {
    pub amount: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<SourceOrderItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<Shipping>,
}
impl std::fmt::Display for SourceOrder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}