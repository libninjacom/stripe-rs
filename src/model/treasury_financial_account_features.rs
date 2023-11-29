
use serde::{Serialize, Deserialize};
use super::{
    TreasuryFinancialAccountsResourceFinancialAddressesFeatures,
    TreasuryFinancialAccountsResourceInboundTransfers,
    TreasuryFinancialAccountsResourceOutboundPayments,
    TreasuryFinancialAccountsResourceOutboundTransfers,
    TreasuryFinancialAccountsResourceToggleSettings,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryFinancialAccountFeatures {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<TreasuryFinancialAccountsResourceToggleSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_insurance: Option<TreasuryFinancialAccountsResourceToggleSettings>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses: Option<
        TreasuryFinancialAccountsResourceFinancialAddressesFeatures,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfers: Option<TreasuryFinancialAccountsResourceInboundTransfers>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows: Option<TreasuryFinancialAccountsResourceToggleSettings>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments: Option<TreasuryFinancialAccountsResourceOutboundPayments>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers: Option<TreasuryFinancialAccountsResourceOutboundTransfers>,
}
impl std::fmt::Display for TreasuryFinancialAccountFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}