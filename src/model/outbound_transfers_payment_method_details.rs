
use serde::{Serialize, Deserialize};
use super::{
    OutboundTransfersPaymentMethodDetailsUsBankAccount,
    TreasurySharedResourceBillingDetails,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutboundTransfersPaymentMethodDetails {
    pub billing_details: TreasurySharedResourceBillingDetails,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<OutboundTransfersPaymentMethodDetailsUsBankAccount>,
}
impl std::fmt::Display for OutboundTransfersPaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}