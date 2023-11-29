
use serde::{Serialize, Deserialize};
use super::IssuingNetworkTokenAddress;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingNetworkTokenWalletProvider {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_trust_score: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_number_source: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_address: Option<IssuingNetworkTokenAddress>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_trust_score: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashed_account_email_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_codes: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_decision: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_decision_version: Option<String>,
}
impl std::fmt::Display for IssuingNetworkTokenWalletProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}