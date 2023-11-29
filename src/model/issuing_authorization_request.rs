
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingAuthorizationRequest {
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_details: Option<serde_json::Value>,
    pub approved: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<String>,
    pub created: i64,
    pub currency: String,
    pub merchant_amount: i64,
    pub merchant_currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_risk_score: Option<i64>,
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_message: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<i64>,
}
impl std::fmt::Display for IssuingAuthorizationRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}