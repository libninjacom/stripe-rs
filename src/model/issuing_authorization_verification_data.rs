
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingAuthorizationVerificationData {
    pub address_line1_check: String,
    pub address_postal_code_check: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_exemption: Option<serde_json::Value>,
    pub cvc_check: String,
    pub expiry_check: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<serde_json::Value>,
}
impl std::fmt::Display for IssuingAuthorizationVerificationData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}