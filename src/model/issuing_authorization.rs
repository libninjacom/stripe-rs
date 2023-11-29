
use serde::{Serialize, Deserialize};
use super::{
    BalanceTransaction, IssuingAuthorizationMerchantData, IssuingAuthorizationRequest,
    IssuingAuthorizationVerificationData, IssuingCard, IssuingTransaction,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuingAuthorization {
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_details: Option<serde_json::Value>,
    pub approved: bool,
    pub authorization_method: String,
    pub balance_transactions: Vec<BalanceTransaction>,
    pub card: IssuingCard,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<serde_json::Value>,
    pub created: i64,
    pub currency: String,
    pub id: String,
    pub livemode: bool,
    pub merchant_amount: i64,
    pub merchant_currency: String,
    pub merchant_data: IssuingAuthorizationMerchantData,
    pub metadata: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_data: Option<serde_json::Value>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_request: Option<serde_json::Value>,
    pub request_history: Vec<IssuingAuthorizationRequest>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<serde_json::Value>,
    pub transactions: Vec<IssuingTransaction>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<serde_json::Value>,
    pub verification_data: IssuingAuthorizationVerificationData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
}
impl std::fmt::Display for IssuingAuthorization {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}