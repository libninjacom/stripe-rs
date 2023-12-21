use serde::{Serialize, Deserialize};
use super::{
    CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft,
    CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction,
    CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction,
    CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction,
    CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance,
    CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction,
};
/**Customers with certain payments enabled have a cash balance, representing funds that were paid
by the customer to a merchant, but have not yet been allocated to a payment. Cash Balance Transactions
represent when funds are moved into or out of this balance. This includes funding by the customer, allocation
to payments, and refunds to the customer.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerCashBalanceTransaction {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjusted_for_overdraft: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceAdjustedForOverdraft,
    >,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applied_to_payment: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction,
    >,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///The customer whose available cash balance changed as a result of this transaction.
    pub customer: serde_json::Value,
    ///The total available cash balance for the specified currency after this transaction was applied. Represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub ending_balance: i64,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funded: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction,
    >,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///The amount by which the cash balance changed, represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal). A positive value represents funds being added to the cash balance, a negative value represents funds being removed from the cash balance.
    pub net_amount: i64,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunded_from_payment: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction,
    >,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transferred_to_balance: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceTransferredToBalance,
    >,
    ///The type of the cash balance transaction. New types may be added in future. See [Customer Balance](https://stripe.com/docs/payments/customer-balance#types) to learn more about these types.
    #[serde(rename = "type")]
    pub type_: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unapplied_from_payment: Option<
        CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction,
    >,
}
impl std::fmt::Display for CustomerCashBalanceTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}