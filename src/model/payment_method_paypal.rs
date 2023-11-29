
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodPaypal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payer_id: Option<String>,
}
impl std::fmt::Display for PaymentMethodPaypal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}