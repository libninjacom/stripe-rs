use serde::{Serialize, Deserialize};
use super::{
    TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions,
    TreasuryTransactionsResourceBalanceImpact,
    TreasuryTransactionsResourceTransactionEntryList,
};
///Transactions represent changes to a [FinancialAccount's](https://stripe.com/docs/api#financial_accounts) balance.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TreasuryTransaction {
    ///Amount (in cents) transferred.
    pub amount: i64,
    ///Change to a FinancialAccount's balance
    pub balance_impact: TreasuryTransactionsResourceBalanceImpact,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    pub description: String,
    ///A list of TransactionEntries that are part of this Transaction. This cannot be expanded in any list endpoints.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entries: Option<TreasuryTransactionsResourceTransactionEntryList>,
    ///The FinancialAccount associated with this object.
    pub financial_account: String,
    ///ID of the flow that created the Transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<String>,
    ///Details of the flow that created the Transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_details: Option<serde_json::Value>,
    ///Type of the flow that created the Transaction.
    pub flow_type: String,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Status of the Transaction.
    pub status: String,
    ///
    pub status_transitions: TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions,
}
impl std::fmt::Display for TreasuryTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}