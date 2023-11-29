
use serde::{Serialize, Deserialize};
use super::{
    BankConnectionsResourceLinkAccountSessionFilters,
    BankConnectionsResourceLinkedAccountList,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinancialConnectionsSession {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder: Option<serde_json::Value>,
    pub accounts: BankConnectionsResourceLinkedAccountList,
    pub client_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<BankConnectionsResourceLinkAccountSessionFilters>,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub permissions: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}
impl std::fmt::Display for FinancialConnectionsSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}