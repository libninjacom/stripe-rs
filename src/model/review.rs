use serde::{Serialize, Deserialize};
/**Reviews can be used to supplement automated fraud detection with human expertise.

Learn more about [Radar](/radar) and reviewing payments
[here](https://stripe.com/docs/radar/reviews).*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Review {
    ///The ZIP or postal code of the card used, if applicable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_zip: Option<String>,
    ///The charge associated with this review.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<serde_json::Value>,
    ///The reason the review was closed, or null if it has not yet been closed. One of `approved`, `refunded`, `refunded_as_fraud`, `disputed`, or `redacted`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub closed_reason: Option<String>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Unique identifier for the object.
    pub id: String,
    ///The IP address where the payment originated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    ///Information related to the location of the payment. Note that this information is an approximation and attempts to locate the nearest population center - it should not be used to determine a specific address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address_location: Option<serde_json::Value>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///If `true`, the review needs action.
    pub open: bool,
    ///The reason the review was opened. One of `rule` or `manual`.
    pub opened_reason: String,
    ///The PaymentIntent ID associated with this review, if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<serde_json::Value>,
    ///The reason the review is currently open or closed. One of `rule`, `manual`, `approved`, `refunded`, `refunded_as_fraud`, `disputed`, or `redacted`.
    pub reason: String,
    ///Information related to the browsing session of the user who initiated the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<serde_json::Value>,
}
impl std::fmt::Display for Review {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}