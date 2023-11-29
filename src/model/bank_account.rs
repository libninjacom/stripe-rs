
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_payout_methods: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    pub country: String,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_currency: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<serde_json::Value>,
    pub id: String,
    pub last4: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
    pub status: String,
}
impl std::fmt::Display for BankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}