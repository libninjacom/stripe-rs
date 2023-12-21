use serde::{Serialize, Deserialize};
use super::TransferReversalList;
/**A `Transfer` object is created when you move funds between Stripe accounts as
part of Connect.

Before April 6, 2017, transfers also represented movement of funds from a
Stripe account to a card or bank account. This behavior has since been split
out into a [Payout](https://stripe.com/docs/api#payout_object) object, with corresponding payout endpoints. For more
information, read about the
[transfer/payout split](https://stripe.com/docs/transfer-payout-split).

Related guide: [Creating separate charges and transfers](https://stripe.com/docs/connect/separate-charges-and-transfers)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Transfer {
    ///Amount in cents (or local equivalent) to be transferred.
    pub amount: i64,
    ///Amount in cents (or local equivalent) reversed (can be less than the amount attribute on the transfer if a partial reversal was issued).
    pub amount_reversed: i64,
    ///Balance transaction that describes the impact of this transfer on your account balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<serde_json::Value>,
    ///Time that this record of the transfer was first created.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///ID of the Stripe account the transfer was sent to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<serde_json::Value>,
    ///If the destination is a Stripe account, this will be the ID of the payment that the destination account received for the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment: Option<serde_json::Value>,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///A list of reversals that have been applied to the transfer.
    pub reversals: TransferReversalList,
    ///Whether the transfer has been fully reversed. If the transfer is only partially reversed, this attribute will still be false.
    pub reversed: bool,
    ///ID of the charge or payment that was used to fund the transfer. If null, the transfer was funded from the available balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transaction: Option<serde_json::Value>,
    ///The source balance this transfer came from. One of `card`, `fpx`, or `bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,
    ///A string that identifies this transaction as part of a group. See the [Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers#transfer-options) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}
impl std::fmt::Display for Transfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}