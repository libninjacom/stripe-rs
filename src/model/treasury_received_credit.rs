
use serde::{Serialize, Deserialize};
use super::{
    TreasuryReceivedCreditsResourceLinkedFlows,
    TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryReceivedCredit {
    pub amount: i64,
    pub created: i64,
    pub currency: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_regulatory_receipt_url: Option<String>,
    pub id: String,
    pub initiating_payment_method_details: TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails,
    pub linked_flows: TreasuryReceivedCreditsResourceLinkedFlows,
    pub livemode: bool,
    pub network: String,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversal_details: Option<serde_json::Value>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<serde_json::Value>,
}
impl std::fmt::Display for TreasuryReceivedCredit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}