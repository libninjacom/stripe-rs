use serde::{Serialize, Deserialize};
/**[Stripe Connect](https://stripe.com/docs/connect) platforms can reverse transfers made to a
connected account, either entirely or partially, and can also specify whether
to refund any related application fees. Transfer reversals add to the
platform's balance and subtract from the destination account's balance.

Reversing a transfer that was made for a [destination
charge](/docs/connect/destination-charges) is allowed only up to the amount of
the charge. It is possible to reverse a
[transfer_group](https://stripe.com/docs/connect/separate-charges-and-transfers#transfer-options)
transfer only if the destination account has enough balance to cover the
reversal.

Related guide: [Reversing transfers](https://stripe.com/docs/connect/separate-charges-and-transfers#reversing-transfers)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TransferReversal {
    ///Amount, in cents (or local equivalent).
    pub amount: i64,
    ///Balance transaction that describes the impact on your account balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<serde_json::Value>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///Linked payment refund for the transfer reversal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment_refund: Option<serde_json::Value>,
    ///Unique identifier for the object.
    pub id: String,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///ID of the refund responsible for the transfer reversal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_refund: Option<serde_json::Value>,
    ///ID of the transfer that was reversed.
    pub transfer: serde_json::Value,
}
impl std::fmt::Display for TransferReversal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}