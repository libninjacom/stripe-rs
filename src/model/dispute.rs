
use serde::{Serialize, Deserialize};
use super::{
    BalanceTransaction, DisputeEvidence, DisputeEvidenceDetails,
    DisputePaymentMethodDetails,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dispute {
    pub amount: i64,
    pub balance_transactions: Vec<BalanceTransaction>,
    pub charge: serde_json::Value,
    pub created: i64,
    pub currency: String,
    pub evidence: DisputeEvidence,
    pub evidence_details: DisputeEvidenceDetails,
    pub id: String,
    pub is_charge_refundable: bool,
    pub livemode: bool,
    pub metadata: serde_json::Value,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_details: Option<DisputePaymentMethodDetails>,
    pub reason: String,
    pub status: String,
}
impl std::fmt::Display for Dispute {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}