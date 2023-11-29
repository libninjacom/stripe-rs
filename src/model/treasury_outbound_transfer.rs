
use serde::{Serialize, Deserialize};
use super::{
    OutboundTransfersPaymentMethodDetails,
    TreasuryOutboundTransfersResourceStatusTransitions,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryOutboundTransfer {
    pub amount: i64,
    pub cancelable: bool,
    pub created: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment_method: Option<String>,
    pub destination_payment_method_details: OutboundTransfersPaymentMethodDetails,
    pub expected_arrival_date: i64,
    pub financial_account: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_regulatory_receipt_url: Option<String>,
    pub id: String,
    pub livemode: bool,
    pub metadata: serde_json::Value,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_details: Option<serde_json::Value>,
    pub statement_descriptor: String,
    pub status: String,
    pub status_transitions: TreasuryOutboundTransfersResourceStatusTransitions,
    pub transaction: serde_json::Value,
}
impl std::fmt::Display for TreasuryOutboundTransfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}