
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceTransactionChfCreditTransferData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_address_country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_address_line1: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_iban: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
}
impl std::fmt::Display for SourceTransactionChfCreditTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}