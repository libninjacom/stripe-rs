
use serde::{Serialize, Deserialize};
use super::{
    TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows,
    TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryInboundTransfer {
    pub amount: i64,
    pub cancelable: bool,
    pub created: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<serde_json::Value>,
    pub financial_account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_regulatory_receipt_url: Option<String>,
    pub id: String,
    pub linked_flows: TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows,
    pub livemode: bool,
    pub metadata: serde_json::Value,
    pub object: String,
    pub origin_payment_method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_payment_method_details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned: Option<bool>,
    pub statement_descriptor: String,
    pub status: String,
    pub status_transitions: TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<serde_json::Value>,
}
impl std::fmt::Display for TreasuryInboundTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}