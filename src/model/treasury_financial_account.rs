use serde::{Serialize, Deserialize};
use super::{
    TreasuryFinancialAccountFeatures, TreasuryFinancialAccountsResourceBalance,
    TreasuryFinancialAccountsResourceFinancialAddress,
    TreasuryFinancialAccountsResourceStatusDetails,
};
/**Stripe Treasury provides users with a container for money called a FinancialAccount that is separate from their Payments balance.
FinancialAccounts serve as the source and destination of Treasury’s money movement APIs.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryFinancialAccount {
    ///The array of paths to active Features in the Features hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_features: Option<Vec<String>>,
    ///Balance information for the FinancialAccount
    pub balance: TreasuryFinancialAccountsResourceBalance,
    ///Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    /**Encodes whether a FinancialAccount has access to a particular Feature, with a `status` enum and associated `status_details`.
Stripe or the platform can control Features via the requested field.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<TreasuryFinancialAccountFeatures>,
    ///The set of credentials that resolve to a FinancialAccount.
    pub financial_addresses: Vec<TreasuryFinancialAccountsResourceFinancialAddress>,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The array of paths to pending Features in the Features hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_features: Option<Vec<String>>,
    ///The set of functionalities that the platform can restrict on the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_restrictions: Option<serde_json::Value>,
    ///The array of paths to restricted Features in the Features hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_features: Option<Vec<String>>,
    ///The enum specifying what state the account is in.
    pub status: String,
    ///
    pub status_details: TreasuryFinancialAccountsResourceStatusDetails,
    ///The currencies the FinancialAccount can hold a balance in. Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    pub supported_currencies: Vec<String>,
}
impl std::fmt::Display for TreasuryFinancialAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}