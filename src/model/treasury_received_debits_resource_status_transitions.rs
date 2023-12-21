use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryReceivedDebitsResourceStatusTransitions {
    ///Timestamp describing when the DebitReversal changed status to `completed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
}
impl std::fmt::Display for TreasuryReceivedDebitsResourceStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}