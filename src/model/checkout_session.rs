use serde::{Serialize, Deserialize};
use super::{
    PaymentPagesCheckoutSessionAutomaticTax, PaymentPagesCheckoutSessionCustomFields,
    PaymentPagesCheckoutSessionCustomText, PaymentPagesCheckoutSessionListLineItems,
    PaymentPagesCheckoutSessionPhoneNumberCollection,
    PaymentPagesCheckoutSessionShippingOption, PaymentPagesCheckoutSessionTaxIdCollection,
};
/**A Checkout Session represents your customer's session as they pay for
one-time purchases or subscriptions through [Checkout](https://stripe.com/docs/payments/checkout)
or [Payment Links](https://stripe.com/docs/payments/payment-links). We recommend creating a
new Session each time your customer attempts to pay.

Once payment is successful, the Checkout Session will contain a reference
to the [Customer](https://stripe.com/docs/api/customers), and either the successful
[PaymentIntent](https://stripe.com/docs/api/payment_intents) or an active
[Subscription](https://stripe.com/docs/api/subscriptions).

You can create a Checkout Session on your server and redirect to its URL
to begin Checkout.

Related guide: [Checkout quickstart](https://stripe.com/docs/checkout/quickstart)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CheckoutSession {
    ///When set, provides configuration for actions to take if this Checkout Session expires.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_expiration: Option<serde_json::Value>,
    ///Enables user redeemable promotion codes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,
    ///Total of all items before discounts or taxes are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_subtotal: Option<i64>,
    ///Total of all items after discounts and taxes are applied.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_total: Option<i64>,
    ///
    pub automatic_tax: PaymentPagesCheckoutSessionAutomaticTax,
    ///Describes whether Checkout should collect the customer's billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<String>,
    ///If set, Checkout displays a back button and customers will be directed to this URL if they decide to cancel payment and return to your website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    /**A unique string to reference the Checkout Session. This can be a
customer ID, a cart ID, or similar, and can be used to reconcile the
Session with your internal systems.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<String>,
    ///Client secret to be used when initializing Stripe.js embedded checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    ///Results of `consent_collection` for this session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent: Option<serde_json::Value>,
    ///When set, provides configuration for the Checkout Session to gather active consent from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_collection: Option<serde_json::Value>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    ///Currency conversion details for automatic currency conversion sessions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_conversion: Option<serde_json::Value>,
    ///Collect additional information from your customer using custom fields. Up to 2 fields are supported.
    pub custom_fields: Vec<PaymentPagesCheckoutSessionCustomFields>,
    ///
    pub custom_text: PaymentPagesCheckoutSessionCustomText,
    /**The ID of the customer for this Session.
For Checkout Sessions in `subscription` mode or Checkout Sessions with `customer_creation` set as `always` in `payment` mode, Checkout
will create a new customer object based on information provided
during the payment flow unless an existing customer was provided when
the Session was created.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    ///Configure whether a Checkout Session creates a Customer when the Checkout Session completes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_creation: Option<String>,
    ///The customer details including the customer's tax exempt status and the customer's tax IDs. Only the customer's email is present on Sessions in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_details: Option<serde_json::Value>,
    /**If provided, this value will be used when the Customer object is created.
If not provided, customers will be asked to enter their email address.
Use this parameter to prefill customer data if you already have an email
on file. To access information about the customer once the payment flow is
complete, use the `customer` attribute.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,
    ///The timestamp at which the Checkout Session will expire.
    pub expires_at: i64,
    ///Unique identifier for the object.
    pub id: String,
    ///ID of the invoice created by the Checkout Session, if it exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<serde_json::Value>,
    ///Details on the state of invoice creation for the Checkout Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_creation: Option<serde_json::Value>,
    ///The line items purchased by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<PaymentPagesCheckoutSessionListLineItems>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///The IETF language tag of the locale Checkout is displayed in. If blank or `auto`, the browser's locale is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///The mode of the Checkout Session.
    pub mode: String,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The ID of the PaymentIntent for Checkout Sessions in `payment` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<serde_json::Value>,
    ///The ID of the Payment Link that created this Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_link: Option<serde_json::Value>,
    ///Configure whether a Checkout Session should collect a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_collection: Option<String>,
    ///Information about the payment method configuration used for this Checkout session if using dynamic payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration_details: Option<serde_json::Value>,
    ///Payment-method-specific configuration for the PaymentIntent or SetupIntent of this CheckoutSession.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<serde_json::Value>,
    /**A list of the types of payment methods (e.g. card) this Checkout
Session is allowed to accept.*/
    pub payment_method_types: Vec<String>,
    /**The payment status of the Checkout Session, one of `paid`, `unpaid`, or `no_payment_required`.
You can use this value to decide when to fulfill your customer's order.*/
    pub payment_status: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_collection: Option<
        PaymentPagesCheckoutSessionPhoneNumberCollection,
    >,
    ///The ID of the original expired Checkout Session that triggered the recovery flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovered_from: Option<String>,
    ///Applies to Checkout Sessions with `ui_mode: embedded`. By default, Stripe will always redirect to your return_url after a successful confirmation. If you set `redirect_on_completion: 'if_required'`, then we will only redirect if your user chooses a redirect-based payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_on_completion: Option<String>,
    ///Applies to Checkout Sessions with `ui_mode: embedded`. The URL to redirect your customer back to after they authenticate or cancel their payment on the payment method's app or site.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    ///The ID of the SetupIntent for Checkout Sessions in `setup` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_intent: Option<serde_json::Value>,
    ///When set, provides configuration for Checkout to collect a shipping address from a customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<serde_json::Value>,
    ///The details of the customer cost of shipping, including the customer chosen ShippingRate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<serde_json::Value>,
    ///Shipping information for this Checkout Session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<serde_json::Value>,
    ///The shipping rate options applied to this Session.
    pub shipping_options: Vec<PaymentPagesCheckoutSessionShippingOption>,
    ///The status of the Checkout Session, one of `open`, `complete`, or `expired`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /**Describes the type of transaction being performed by Checkout in order to customize
relevant text on the page, such as the submit button. `submit_type` can only be
specified on Checkout Sessions in `payment` mode, but not Checkout Sessions
in `subscription` or `setup` mode.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<String>,
    ///The ID of the subscription for Checkout Sessions in `subscription` mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<serde_json::Value>,
    /**The URL the customer will be directed to after the payment or
subscription creation is successful.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_collection: Option<PaymentPagesCheckoutSessionTaxIdCollection>,
    ///Tax and discount details for the computed total amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_details: Option<serde_json::Value>,
    ///The UI mode of the Session. Can be `hosted` (default) or `embedded`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_mode: Option<String>,
    /**The URL to the Checkout Session. Redirect customers to this URL to take them to Checkout. If you’re using [Custom Domains](https://stripe.com/docs/payments/checkout/custom-domains), the URL will use your subdomain. Otherwise, it’ll use `checkout.stripe.com.`
This value is only present when the session is active.*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl std::fmt::Display for CheckoutSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}