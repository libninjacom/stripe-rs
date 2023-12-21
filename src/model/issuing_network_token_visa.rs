use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingNetworkTokenVisa {
    ///A unique reference ID from Visa to represent the card account number.
    pub card_reference_id: String,
    ///The network-unique identifier for the token.
    pub token_reference_id: String,
    ///The ID of the entity requesting tokenization, specific to Visa.
    pub token_requestor_id: String,
    ///Degree of risk associated with the token between `01` and `99`, with higher number indicating higher risk. A `00` value indicates the token was not scored by Visa.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token_risk_score: Option<String>,
}
impl std::fmt::Display for IssuingNetworkTokenVisa {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}