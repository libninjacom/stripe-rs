
use serde::{Serialize, Deserialize};
use super::{
    TreasuryFinancialAccountFeatures, TreasuryFinancialAccountsResourceBalance,
    TreasuryFinancialAccountsResourceFinancialAddress,
    TreasuryFinancialAccountsResourceStatusDetails,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TreasuryFinancialAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_features: Option<Vec<String>>,
    pub balance: TreasuryFinancialAccountsResourceBalance,
    pub country: String,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<TreasuryFinancialAccountFeatures>,
    pub financial_addresses: Vec<TreasuryFinancialAccountsResourceFinancialAddress>,
    pub id: String,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_features: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_restrictions: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_features: Option<Vec<String>>,
    pub status: String,
    pub status_details: TreasuryFinancialAccountsResourceStatusDetails,
    pub supported_currencies: Vec<String>,
}
impl std::fmt::Display for TreasuryFinancialAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}