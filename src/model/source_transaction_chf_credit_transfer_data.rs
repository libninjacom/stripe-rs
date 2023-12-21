use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceTransactionChfCreditTransferData {
    ///Reference associated with the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
    ///Sender's country address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_address_country: Option<String>,
    ///Sender's line 1 address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_address_line1: Option<String>,
    ///Sender's bank account IBAN.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_iban: Option<String>,
    ///Sender's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sender_name: Option<String>,
}
impl std::fmt::Display for SourceTransactionChfCreditTransferData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}