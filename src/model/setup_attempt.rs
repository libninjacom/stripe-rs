use serde::{Serialize, Deserialize};
use super::SetupAttemptPaymentMethodDetails;
/**A SetupAttempt describes one attempted confirmation of a SetupIntent,
whether that confirmation is successful or unsuccessful. You can use
SetupAttempts to inspect details of a specific attempt at setting up a
payment method using a SetupIntent.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetupAttempt {
    ///The value of [application](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-application) on the SetupIntent at the time of this confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<serde_json::Value>,
    /**If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.

It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers. It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_to_self: Option<bool>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///The value of [customer](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-customer) on the SetupIntent at the time of this confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    /**Indicates the directions of money movement for which this payment method is intended to be used.

Include `inbound` if you intend to use the payment method as the origin to pull funds from. Include `outbound` if you intend to use the payment method as the destination to send funds to. You can include both if you intend to use the payment method for both purposes.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_directions: Option<Vec<String>>,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The value of [on_behalf_of](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-on_behalf_of) on the SetupIntent at the time of this confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<serde_json::Value>,
    ///ID of the payment method used with this SetupAttempt.
    pub payment_method: serde_json::Value,
    ///
    pub payment_method_details: SetupAttemptPaymentMethodDetails,
    ///The error encountered during this attempt to confirm the SetupIntent, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_error: Option<serde_json::Value>,
    ///ID of the SetupIntent that this attempt belongs to.
    pub setup_intent: serde_json::Value,
    ///Status of this SetupAttempt, one of `requires_confirmation`, `requires_action`, `processing`, `succeeded`, `failed`, or `abandoned`.
    pub status: String,
    ///The value of [usage](https://stripe.com/docs/api/setup_intents/object#setup_intent_object-usage) on the SetupIntent at the time of this confirmation, one of `off_session` or `on_session`.
    pub usage: String,
}
impl std::fmt::Display for SetupAttempt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}