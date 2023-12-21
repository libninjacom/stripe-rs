use serde::{Serialize, Deserialize};
use super::TreasuryTransactionsResourceBalanceImpact;
///TransactionEntries represent individual units of money movements within a single [Transaction](https://stripe.com/docs/api#transactions).
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryTransactionEntry {
    ///Change to a FinancialAccount's balance
    pub balance_impact: TreasuryTransactionsResourceBalanceImpact,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///When the TransactionEntry will impact the FinancialAccount's balance.
    pub effective_at: i64,
    ///The FinancialAccount associated with this object.
    pub financial_account: String,
    ///Token of the flow associated with the TransactionEntry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    ///Details of the flow associated with the TransactionEntry.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_details: Option<serde_json::Value>,
    ///Type of the flow associated with the TransactionEntry.
    pub flow_type: String,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The Transaction associated with this object.
    pub transaction: serde_json::Value,
    ///The specific money movement that generated the TransactionEntry.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for TreasuryTransactionEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}