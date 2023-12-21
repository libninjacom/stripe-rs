use serde::{Serialize, Deserialize};
use super::{BillingDetails, RadarRadarOptions, RefundList};
/**The `Charge` object represents a single attempt to move money into your Stripe account.
PaymentIntent confirmation is the most common way to create Charges, but transferring
money to a different Stripe account through Connect also creates Charges.
Some legacy payment flows create Charges directly, which is not recommended for new integrations.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Charge {
    ///Amount intended to be collected by this payment. A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency). The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts). The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub amount: i64,
    ///Amount in cents (or local equivalent) captured (can be less than the amount attribute on the charge if a partial capture was made).
    pub amount_captured: i64,
    ///Amount in cents (or local equivalent) refunded (can be less than the amount attribute on the charge if a partial refund was issued).
    pub amount_refunded: i64,
    ///ID of the Connect application that created the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<serde_json::Value>,
    ///The application fee (if any) for the charge. [See the Connect documentation](https://stripe.com/docs/connect/direct-charges#collecting-fees) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<serde_json::Value>,
    ///The amount of the application fee (if any) requested for the charge. [See the Connect documentation](https://stripe.com/docs/connect/direct-charges#collecting-fees) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    ///ID of the balance transaction that describes the impact of this charge on your account balance (not including refunds or disputes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance_transaction: Option<serde_json::Value>,
    ///
    pub billing_details: BillingDetails,
    ///The full statement descriptor that is passed to card networks, and that is displayed on your customers' credit card and bank statements. Allows you to see what the statement descriptor looks like after the static and dynamic portions are combined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub calculated_statement_descriptor: Option<String>,
    ///If the charge was created without capturing, this Boolean represents whether it is still uncaptured or has since been captured.
    pub captured: bool,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///ID of the customer this charge is for if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///Whether the charge has been disputed.
    pub disputed: bool,
    ///ID of the balance transaction that describes the reversal of the balance on your account due to payment failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_balance_transaction: Option<serde_json::Value>,
    ///Error code explaining reason for charge failure if available (see [the errors section](https://stripe.com/docs/error-codes) for a list of codes).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_code: Option<String>,
    ///Message to user further explaining reason for charge failure if available.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_message: Option<String>,
    ///Information on fraud assessments for the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraud_details: Option<serde_json::Value>,
    ///Unique identifier for the object.
    pub id: String,
    ///ID of the invoice this charge is for if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<serde_json::Value>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The account (if any) the charge was made on behalf of without triggering an automatic transfer. See the [Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<serde_json::Value>,
    ///Details about whether the payment was accepted, and why. See [understanding declines](https://stripe.com/docs/declines) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outcome: Option<serde_json::Value>,
    ///`true` if the charge succeeded, or was successfully authorized for later capture.
    pub paid: bool,
    ///ID of the PaymentIntent associated with this charge, if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<serde_json::Value>,
    ///ID of the payment method used in this charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,
    ///Details about the payment method at the time of the transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_details: Option<serde_json::Value>,
    ///Options to configure Radar. See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<RadarRadarOptions>,
    ///This is the email address that the receipt for this charge was sent to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,
    ///This is the transaction number that appears on email receipts sent for this charge. This attribute will be `null` until a receipt has been sent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_number: Option<String>,
    ///This is the URL to view the receipt for this charge. The receipt is kept up-to-date to the latest state of the charge, including any refunds. If the charge is for an Invoice, the receipt will be stylized as an Invoice receipt.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_url: Option<String>,
    ///Whether the charge has been fully refunded. If the charge is only partially refunded, this attribute will still be false.
    pub refunded: bool,
    ///A list of refunds that have been applied to the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refunds: Option<RefundList>,
    ///ID of the review associated with this charge if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review: Option<serde_json::Value>,
    ///Shipping information for the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<serde_json::Value>,
    ///The transfer ID which created this charge. Only present if the charge came from another Stripe account. [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transfer: Option<serde_json::Value>,
    ///For card charges, use `statement_descriptor_suffix` instead. Otherwise, you can use this value as the complete description of a charge on your customers’ statements. Must contain at least one letter, maximum 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    ///Provides information about the charge that customers see on their statements. Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor. Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,
    ///The status of the payment is either `succeeded`, `pending`, or `failed`.
    pub status: String,
    ///ID of the transfer to the `destination` account (only applicable if the charge was created using the `destination` parameter).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<serde_json::Value>,
    ///An optional dictionary including the account to automatically transfer to as part of a destination charge. [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<serde_json::Value>,
    ///A string that identifies this transaction as part of a group. See the [Connect documentation](https://stripe.com/docs/connect/separate-charges-and-transfers#transfer-options) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}
impl std::fmt::Display for Charge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}