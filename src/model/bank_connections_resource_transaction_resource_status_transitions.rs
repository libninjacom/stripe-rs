use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BankConnectionsResourceTransactionResourceStatusTransitions {
    ///Time at which this transaction posted. Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<i64>,
    ///Time at which this transaction was voided. Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub void_at: Option<i64>,
}
impl std::fmt::Display for BankConnectionsResourceTransactionResourceStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}