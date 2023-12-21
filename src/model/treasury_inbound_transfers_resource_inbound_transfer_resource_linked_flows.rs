use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows {
    ///If funds for this flow were returned after the flow went to the `succeeded` state, this field contains a reference to the ReceivedDebit return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_debit: Option<String>,
}
impl std::fmt::Display
for TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}