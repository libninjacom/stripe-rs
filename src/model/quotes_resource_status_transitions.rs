use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct QuotesResourceStatusTransitions {
    ///The time that the quote was accepted. Measured in seconds since Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub accepted_at: Option<i64>,
    ///The time that the quote was canceled. Measured in seconds since Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,
    ///The time that the quote was finalized. Measured in seconds since Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub finalized_at: Option<i64>,
}
impl std::fmt::Display for QuotesResourceStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}