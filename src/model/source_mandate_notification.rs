use serde::{Serialize, Deserialize};
use super::{
    Source, SourceMandateNotificationAcssDebitData,
    SourceMandateNotificationBacsDebitData, SourceMandateNotificationSepaDebitData,
};
/**Source mandate notifications should be created when a notification related to
a source mandate must be sent to the payer. They will trigger a webhook or
deliver an email to the customer.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceMandateNotification {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<SourceMandateNotificationAcssDebitData>,
    ///A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the amount associated with the mandate notification. The amount is expressed in the currency of the underlying source. Required if the notification type is `debit_initiated`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<SourceMandateNotificationBacsDebitData>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The reason of the mandate notification. Valid reasons are `mandate_confirmed` or `debit_initiated`.
    pub reason: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<SourceMandateNotificationSepaDebitData>,
    /**`Source` objects allow you to accept a variety of payment methods. They
represent a customer's payment instrument, and can be used with the Stripe API
just like a `Card` object: once chargeable, they can be charged, or can be
attached to customers.

Stripe doesn't recommend using the deprecated [Sources API](https://stripe.com/docs/api/sources).
We recommend that you adopt the [PaymentMethods API](https://stripe.com/docs/api/payment_methods).
This newer API provides access to our latest features and payment method types.

Related guides: [Sources API](https://stripe.com/docs/sources) and [Sources & Customers](https://stripe.com/docs/sources/customers).*/
    pub source: Source,
    ///The status of the mandate notification. Valid statuses are `pending` or `submitted`.
    pub status: String,
    ///The type of source this mandate notification is attached to. Should be the source type identifier code for the payment method, such as `three_d_secure`.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for SourceMandateNotification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}