use serde::{Serialize, Deserialize};
use super::IssuingNetworkTokenAddress;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingNetworkTokenWalletProvider {
    ///The wallet provider-given account ID of the digital wallet the token belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_id: Option<String>,
    ///An evaluation on the trustworthiness of the wallet account between 1 and 5. A higher score indicates more trustworthy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_trust_score: Option<i64>,
    ///The method used for tokenizing a card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_number_source: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_address: Option<IssuingNetworkTokenAddress>,
    ///The name of the cardholder tokenizing the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_name: Option<String>,
    ///An evaluation on the trustworthiness of the device. A higher score indicates more trustworthy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_trust_score: Option<i64>,
    ///The hashed email address of the cardholder's account with the wallet provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hashed_account_email_address: Option<String>,
    ///The reasons for suggested tokenization given by the card network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason_codes: Option<Vec<String>>,
    ///The recommendation on responding to the tokenization request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_decision: Option<String>,
    ///The version of the standard for mapping reason codes followed by the wallet provider.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suggested_decision_version: Option<String>,
}
impl std::fmt::Display for IssuingNetworkTokenWalletProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}