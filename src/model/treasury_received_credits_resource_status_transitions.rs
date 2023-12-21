use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryReceivedCreditsResourceStatusTransitions {
    ///Timestamp describing when the CreditReversal changed status to `posted`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<i64>,
}
impl std::fmt::Display for TreasuryReceivedCreditsResourceStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}