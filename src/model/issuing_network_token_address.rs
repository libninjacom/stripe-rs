use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingNetworkTokenAddress {
    ///The street address of the cardholder tokenizing the card.
    pub line1: String,
    ///The postal code of the cardholder tokenizing the card.
    pub postal_code: String,
}
impl std::fmt::Display for IssuingNetworkTokenAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}