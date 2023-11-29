
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FinancialConnectionsAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_refresh: Option<serde_json::Value>,
    pub category: String,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,
    pub id: String,
    pub institution_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    pub livemode: bool,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ownership_refresh: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    pub status: String,
    pub subcategory: String,
    pub supported_payment_method_types: Vec<String>,
}
impl std::fmt::Display for FinancialConnectionsAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}