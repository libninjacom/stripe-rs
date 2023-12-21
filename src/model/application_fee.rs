use serde::{Serialize, Deserialize};
use super::FeeRefundList;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplicationFee {
    ///ID of the Stripe account this fee was taken from.
    pub account: serde_json::Value,
    ///Amount earned, in cents (or local equivalent).
    pub amount: i64,
    ///Amount in cents (or local equivalent) refunded (can be less than the amount attribute on the fee if a partial refund was issued)
    pub amount_refunded: i64,
    ///ID of the Connect application that earned the fee.
    pub application: serde_json::Value,
    ///Balance transaction that describes the impact of this collected application fee on your account balance (not including refunds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<serde_json::Value>,
    ///ID of the charge that the application fee was taken from.
    pub charge: serde_json::Value,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///ID of the corresponding charge on the platform account, if this fee was the result of a charge using the `destination` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub originating_transaction: Option<serde_json::Value>,
    ///Whether the fee has been fully refunded. If the fee is only partially refunded, this attribute will still be false.
    pub refunded: bool,
    ///A list of refunds that have been applied to the fee.
    pub refunds: FeeRefundList,
}
impl std::fmt::Display for ApplicationFee {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}