
use serde::{Serialize, Deserialize};
use super::IssuingAuthorizationMerchantData;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuingTransaction {
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<serde_json::Value>,
    pub card: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<serde_json::Value>,
    pub created: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dispute: Option<serde_json::Value>,
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
    pub purchase_details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<serde_json::Value>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
}
impl std::fmt::Display for IssuingTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}