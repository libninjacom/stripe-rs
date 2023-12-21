use serde::{Serialize, Deserialize};
/**A `Payout` object is created when you receive funds from Stripe, or when you
initiate a payout to either a bank account or debit card of a [connected
Stripe account](/docs/connect/bank-debit-card-payouts). You can retrieve individual payouts,
and list all payouts. Payouts are made on [varying
schedules](/docs/connect/manage-payout-schedule), depending on your country and
industry.

Related guide: [Receiving payouts](https://stripe.com/docs/payouts)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Payout {
    ///The amount (in cents (or local equivalent)) that transfers to your bank account or debit card.
    pub amount: i64,
    ///Date that you can expect the payout to arrive in the bank. This factors in delays to account for weekends or bank holidays.
    pub arrival_date: i64,
    ///Returns `true` if the payout is created by an [automated payout schedule](https://stripe.com/docs/payouts#payout-schedule) and `false` if it's [requested manually](https://stripe.com/docs/payouts#manual-payouts).
    pub automatic: bool,
    ///ID of the balance transaction that describes the impact of this payout on your account balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<serde_json::Value>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///ID of the bank account or card the payout is sent to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<serde_json::Value>,
    ///If the payout fails or cancels, this is the ID of the balance transaction that reverses the initial balance transaction and returns the funds from the failed payout back in your balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_balance_transaction: Option<serde_json::Value>,
    ///Error code that provides a reason for a payout failure, if available. View our [list of failure codes](https://stripe.com/docs/api#payout_failures).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    ///Message that provides the reason for a payout failure, if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///The method used to send this payout, which can be `standard` or `instant`. `instant` is supported for payouts to debit cards and bank accounts in certain countries. Learn more about [bank support for Instant Payouts](https://stripe.com/docs/payouts/instant-payouts-banks).
    pub method: String,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///If the payout reverses another, this is the ID of the original payout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_payout: Option<serde_json::Value>,
    ///If `completed`, you can use the [Balance Transactions API](https://stripe.com/docs/api/balance_transactions/list#balance_transaction_list-payout) to list all balance transactions that are paid out in this payout.
    pub reconciliation_status: String,
    ///If the payout reverses, this is the ID of the payout that reverses this payout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reversed_by: Option<serde_json::Value>,
    ///The source balance this payout came from, which can be one of the following: `card`, `fpx`, or `bank_account`.
    pub source_type: String,
    ///Extra information about a payout that displays on the user's bank statement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    ///Current status of the payout: `paid`, `pending`, `in_transit`, `canceled` or `failed`. A payout is `pending` until it's submitted to the bank, when it becomes `in_transit`. The status changes to `paid` if the transaction succeeds, or to `failed` or `canceled` (within 5 business days). Some payouts that fail might initially show as `paid`, then change to `failed`.
    pub status: String,
    ///Can be `bank_account` or `card`.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for Payout {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}