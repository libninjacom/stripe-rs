
use serde::{Serialize, Deserialize};
use super::{IssuingCardAuthorizationControls, IssuingCardholder};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuingCard {
    pub brand: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    pub cardholder: IssuingCardholder,
    pub created: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<String>,
    pub exp_month: i64,
    pub exp_year: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<String>,
    pub id: String,
    pub last4: String,
    pub livemode: bool,
    pub metadata: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replaced_by: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_for: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<serde_json::Value>,
    pub spending_controls: IssuingCardAuthorizationControls,
    pub status: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallets: Option<serde_json::Value>,
}
impl std::fmt::Display for IssuingCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}