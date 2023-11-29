
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLinksResourceCustomText {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_acceptance: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentLinksResourceCustomText {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}