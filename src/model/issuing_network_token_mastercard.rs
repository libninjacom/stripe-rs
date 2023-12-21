use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingNetworkTokenMastercard {
    ///A unique reference ID from MasterCard to represent the card account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_reference_id: Option<String>,
    ///The network-unique identifier for the token.
    pub token_reference_id: String,
    ///The ID of the entity requesting tokenization, specific to MasterCard.
    pub token_requestor_id: String,
    ///The name of the entity requesting tokenization, if known. This is directly provided from MasterCard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_requestor_name: Option<String>,
}
impl std::fmt::Display for IssuingNetworkTokenMastercard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}