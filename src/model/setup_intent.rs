use serde::{Serialize, Deserialize};
/**A SetupIntent guides you through the process of setting up and saving a customer's payment credentials for future payments.
For example, you can use a SetupIntent to set up and save your customer's card without immediately collecting a payment.
Later, you can use [PaymentIntents](https://stripe.com/docs/api#payment_intents) to drive the payment flow.

Create a SetupIntent when you're ready to collect your customer's payment credentials.
Don't maintain long-lived, unconfirmed SetupIntents because they might not be valid.
The SetupIntent transitions through multiple [statuses](https://stripe.com/docs/payments/intents#intent-statuses) as it guides
you through the setup process.

Successful SetupIntents result in payment credentials that are optimized for future payments.
For example, cardholders in [certain regions](/guides/strong-customer-authentication) might need to be run through
[Strong Customer Authentication](https://stripe.com/docs/strong-customer-authentication) during payment method collection
to streamline later [off-session payments](https://stripe.com/docs/payments/setup-intents).
If you use the SetupIntent with a [Customer](https://stripe.com/docs/api#setup_intent_object-customer),
it automatically attaches the resulting payment method to that Customer after successful setup.
We recommend using SetupIntents or [setup_future_usage](https://stripe.com/docs/api#payment_intent_object-setup_future_usage) on
PaymentIntents to save payment methods to prevent saving invalid or unoptimized payment methods.

By using SetupIntents, you can reduce friction for your customers, even as regulations change over time.

Related guide: [Setup Intents API](https://stripe.com/docs/payments/setup-intents)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetupIntent {
    ///ID of the Connect application that created the SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<serde_json::Value>,
    /**If present, the SetupIntent's payment method will be attached to the in-context Stripe Account.

It can only be used for this Stripe Account’s own money movement flows like InboundTransfer and OutboundTransfers. It cannot be set to true when setting up a PaymentMethod for a Customer, and defaults to false when attaching a PaymentMethod to a Customer.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attach_to_self: Option<bool>,
    ///Settings for dynamic payment methods compatible with this Setup Intent
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_payment_methods: Option<serde_json::Value>,
    ///Reason for cancellation of this SetupIntent, one of `abandoned`, `requested_by_customer`, or `duplicate`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    /**The client secret of this SetupIntent. Used for client-side retrieval using a publishable key.

The client secret can be used to complete payment setup from your frontend. It should not be stored, logged, or exposed to anyone other than the customer. Make sure that you have TLS enabled on any page that includes the client secret.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    /**ID of the Customer this SetupIntent belongs to, if one exists.

If present, the SetupIntent's payment method will be attached to the Customer on successful setup. Payment methods attached to other Customers cannot be used with this SetupIntent.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    ///An arbitrary string attached to the object. Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /**Indicates the directions of money movement for which this payment method is intended to be used.

Include `inbound` if you intend to use the payment method as the origin to pull funds from. Include `outbound` if you intend to use the payment method as the destination to send funds to. You can include both if you intend to use the payment method for both purposes.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow_directions: Option<Vec<String>>,
    ///Unique identifier for the object.
    pub id: String,
    ///The error encountered in the previous SetupIntent confirmation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_setup_error: Option<serde_json::Value>,
    ///The most recent SetupAttempt for this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latest_attempt: Option<serde_json::Value>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///ID of the multi use Mandate generated by the SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<serde_json::Value>,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///If present, this property tells you what actions you need to take in order for your customer to continue payment setup.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_action: Option<serde_json::Value>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The account (if any) for which the setup is intended.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<serde_json::Value>,
    ///ID of the payment method used with this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<serde_json::Value>,
    ///Information about the payment method configuration used for this Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration_details: Option<serde_json::Value>,
    ///Payment method-specific configuration for this SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<serde_json::Value>,
    ///The list of payment method types (e.g. card) that this SetupIntent is allowed to set up.
    pub payment_method_types: Vec<String>,
    ///ID of the single_use Mandate generated by the SetupIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_use_mandate: Option<serde_json::Value>,
    ///[Status](https://stripe.com/docs/payments/intents#intent-statuses) of this SetupIntent, one of `requires_payment_method`, `requires_confirmation`, `requires_action`, `processing`, `canceled`, or `succeeded`.
    pub status: String,
    /**Indicates how the payment method is intended to be used in the future.

Use `on_session` if you intend to only reuse the payment method when the customer is in your checkout flow. Use `off_session` if your customer may or may not be in your checkout flow. If not provided, this value defaults to `off_session`.*/
    pub usage: String,
}
impl std::fmt::Display for SetupIntent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}