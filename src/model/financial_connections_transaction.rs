use serde::{Serialize, Deserialize};
use super::BankConnectionsResourceTransactionResourceStatusTransitions;
///A Transaction represents a real transaction that affects a Financial Connections Account balance.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FinancialConnectionsTransaction {
    ///The ID of the Financial Connections Account this transaction belongs to.
    pub account: String,
    ///The amount of this transaction, in cents (or local equivalent).
    pub amount: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///The description of this transaction.
    pub description: String,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The status of the transaction.
    pub status: String,
    ///
    pub status_transitions: BankConnectionsResourceTransactionResourceStatusTransitions,
    ///Time at which the transaction was transacted. Measured in seconds since the Unix epoch.
    pub transacted_at: i64,
    ///The token of the transaction refresh that last updated or created this transaction.
    pub transaction_refresh: String,
    ///Time at which the object was last updated. Measured in seconds since the Unix epoch.
    pub updated: i64,
}
impl std::fmt::Display for FinancialConnectionsTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}