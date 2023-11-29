
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingNetworkTokenMastercard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_reference_id: Option<String>,
    pub token_reference_id: String,
    pub token_requestor_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_requestor_name: Option<String>,
}
impl std::fmt::Display for IssuingNetworkTokenMastercard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}