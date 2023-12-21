use serde::{Serialize, Deserialize};
use super::{
    TreasuryFinancialAccountsResourceFinancialAddressesFeatures,
    TreasuryFinancialAccountsResourceInboundTransfers,
    TreasuryFinancialAccountsResourceOutboundPayments,
    TreasuryFinancialAccountsResourceOutboundTransfers,
    TreasuryFinancialAccountsResourceToggleSettings,
};
/**Encodes whether a FinancialAccount has access to a particular Feature, with a `status` enum and associated `status_details`.
Stripe or the platform can control Features via the requested field.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryFinancialAccountFeatures {
    ///Toggle settings for enabling/disabling a feature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<TreasuryFinancialAccountsResourceToggleSettings>,
    ///Toggle settings for enabling/disabling a feature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_insurance: Option<TreasuryFinancialAccountsResourceToggleSettings>,
    ///Settings related to Financial Addresses features on a Financial Account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses: Option<
        TreasuryFinancialAccountsResourceFinancialAddressesFeatures,
    >,
    ///InboundTransfers contains inbound transfers features for a FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfers: Option<TreasuryFinancialAccountsResourceInboundTransfers>,
    ///Toggle settings for enabling/disabling a feature
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows: Option<TreasuryFinancialAccountsResourceToggleSettings>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Settings related to Outbound Payments features on a Financial Account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments: Option<TreasuryFinancialAccountsResourceOutboundPayments>,
    ///OutboundTransfers contains outbound transfers features for a FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers: Option<TreasuryFinancialAccountsResourceOutboundTransfers>,
}
impl std::fmt::Display for TreasuryFinancialAccountFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}