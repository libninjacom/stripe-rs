use serde::{Serialize, Deserialize};
use super::{
    ReceivedPaymentMethodDetailsFinancialAccount, TreasurySharedResourceBillingDetails,
    TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount,
};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails {
    ///Set when `type` is `balance`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<String>,
    ///
    pub billing_details: TreasurySharedResourceBillingDetails,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<ReceivedPaymentMethodDetailsFinancialAccount>,
    ///Set when `type` is `issuing_card`. This is an [Issuing Card](https://stripe.com/docs/api#issuing_cards) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_card: Option<String>,
    ///Polymorphic type matching the originating money movement's source. This can be an external account, a Stripe balance, or a FinancialAccount.
    #[serde(rename = "type")]
    pub type_: String,
    ///
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