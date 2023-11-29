
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodCard {
    pub brand: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    pub exp_month: i64,
    pub exp_year: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    pub funding: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_from: Option<serde_json::Value>,
    pub last4: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure_usage: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentMethodCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}