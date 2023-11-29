
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodCashapp {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub buyer_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cashtag: Option<String>,
}
impl std::fmt::Display for PaymentMethodCashapp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}