use serde::{Serialize, Deserialize};
/**Each customer has a [Balance](https://stripe.com/docs/api/customers/object#customer_object-balance) value,
which denotes a debit or credit that's automatically applied to their next invoice upon finalization.
You may modify the value directly by using the [update customer API](https://stripe.com/docs/api/customers/update),
or by creating a Customer Balance Transaction, which increments or decrements the customer's `balance` by the specified `amount`.

Related guide: [Customer balance](https://stripe.com/docs/billing/customer/balance)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerBalanceTransaction {
    ///The amount of the transaction. A negative value is a credit for the customer's balance, and a positive value is a debit to the customer's `balance`.
    pub amount: i64,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///The ID of the credit note (if any) related to the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_note: Option<serde_json::Value>,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///The ID of the customer the transaction belongs to.
    pub customer: serde_json::Value,
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The customer's `balance` after the transaction was applied. A negative value decreases the amount due on the customer's next invoice. A positive value increases the amount due on the customer's next invoice.
    pub ending_balance: i64,
    ///Unique identifier for the object.
    pub id: String,
    ///The ID of the invoice (if any) related to the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<serde_json::Value>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Transaction type: `adjustment`, `applied_to_invoice`, `credit_note`, `initial`, `invoice_overpaid`, `invoice_too_large`, `invoice_too_small`, `unspent_receiver_credit`, or `unapplied_from_invoice`. See the [Customer Balance page](https://stripe.com/docs/billing/customer/balance#types) to learn more about transaction types.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for CustomerBalanceTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}