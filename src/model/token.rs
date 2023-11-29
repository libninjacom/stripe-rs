
use serde::{Serialize, Deserialize};
use super::{BankAccount, Card};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Token {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_account: Option<BankAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<Card>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_ip: Option<String>,
    pub created: i64,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    #[serde(rename = "type")]
    pub type_: String,
    pub used: bool,
}
impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}