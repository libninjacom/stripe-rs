
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FinancialConnectionsAccountOwner {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub id: String,
    pub name: String,
    pub object: String,
    pub ownership: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_address: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refreshed_at: Option<i64>,
}
impl std::fmt::Display for FinancialConnectionsAccountOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}