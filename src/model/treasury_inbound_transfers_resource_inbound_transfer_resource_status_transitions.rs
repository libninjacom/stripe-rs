use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions {
    ///Timestamp describing when an InboundTransfer changed status to `canceled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,
    ///Timestamp describing when an InboundTransfer changed status to `failed`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failed_at: Option<i64>,
    ///Timestamp describing when an InboundTransfer changed status to `succeeded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub succeeded_at: Option<i64>,
}
impl std::fmt::Display
for TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}