use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingAuthorizationVerificationData {
    ///Whether the cardholder provided an address first line and if it matched the cardholder’s `billing.address.line1`.
    pub address_line1_check: String,
    ///Whether the cardholder provided a postal code and if it matched the cardholder’s `billing.address.postal_code`.
    pub address_postal_code_check: String,
    ///The exemption applied to this authorization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_exemption: Option<serde_json::Value>,
    ///Whether the cardholder provided a CVC and if it matched Stripe’s record.
    pub cvc_check: String,
    ///Whether the cardholder provided an expiry date and if it matched Stripe’s record.
    pub expiry_check: String,
    ///The postal code submitted as part of the authorization used for postal code verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,
    ///3D Secure details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<serde_json::Value>,
}
impl std::fmt::Display for IssuingAuthorizationVerificationData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}