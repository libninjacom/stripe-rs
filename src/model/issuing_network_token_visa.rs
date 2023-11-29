
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingNetworkTokenVisa {
    pub card_reference_id: String,
    pub token_reference_id: String,
    pub token_requestor_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_risk_score: Option<String>,
}
impl std::fmt::Display for IssuingNetworkTokenVisa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}