
use serde::{Serialize, Deserialize};
use super::{
    ReceivedPaymentMethodDetailsFinancialAccount, TreasurySharedResourceBillingDetails,
    TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
    pub billing_details: TreasurySharedResourceBillingDetails,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<ReceivedPaymentMethodDetailsFinancialAccount>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_card: Option<String>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<
        TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount,
    >,
}
impl std::fmt::Display
for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}