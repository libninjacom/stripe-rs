use serde::{Serialize, Deserialize};
use super::{
    SourceTransactionAchCreditTransferData, SourceTransactionChfCreditTransferData,
    SourceTransactionGbpCreditTransferData, SourceTransactionPaperCheckData,
    SourceTransactionSepaCreditTransferData,
};
/**Some payment methods have no required amount that a customer must send.
Customers can be instructed to send any amount, and it can be made up of
multiple transactions. As such, sources can have multiple associated
transactions.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceTransaction {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<SourceTransactionAchCreditTransferData>,
    ///A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the amount your customer has pushed to the receiver.
    pub amount: i64,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf_credit_transfer: Option<SourceTransactionChfCreditTransferData>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp_credit_transfer: Option<SourceTransactionGbpCreditTransferData>,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_check: Option<SourceTransactionPaperCheckData>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_credit_transfer: Option<SourceTransactionSepaCreditTransferData>,
    ///The ID of the source this transaction is attached to.
    pub source: String,
    ///The status of the transaction, one of `succeeded`, `pending`, or `failed`.
    pub status: String,
    ///The type of source this transaction is attached to.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for SourceTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}