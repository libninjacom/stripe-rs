use serde::{Serialize, Deserialize};
use super::{
    PaymentLinksResourceAfterCompletion, PaymentLinksResourceAutomaticTax,
    PaymentLinksResourceCustomFields, PaymentLinksResourceCustomText,
    PaymentLinksResourceListLineItems, PaymentLinksResourcePhoneNumberCollection,
    PaymentLinksResourceShippingOption, PaymentLinksResourceTaxIdCollection,
};
/**A payment link is a shareable URL that will take your customers to a hosted payment page. A payment link can be shared and used multiple times.

When a customer opens a payment link it will open a new [checkout session](https://stripe.com/docs/api/checkout/sessions) to render the payment page. You can use [checkout session events](https://stripe.com/docs/api/events/types#event_types-checkout.session.completed) to track payments through payment links.

Related guide: [Payment Links API](https://stripe.com/docs/payment-links)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLink {
    ///Whether the payment link's `url` is active. If `false`, customers visiting the URL will be shown a page saying that the link has been deactivated.
    pub active: bool,
    ///
    pub after_completion: PaymentLinksResourceAfterCompletion,
    ///Whether user redeemable promotion codes are enabled.
    pub allow_promotion_codes: bool,
    ///The ID of the Connect application that created the Payment Link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<serde_json::Value>,
    ///The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    ///This represents the percentage of the subscription invoice total that will be transferred to the application owner's Stripe account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    ///
    pub automatic_tax: PaymentLinksResourceAutomaticTax,
    ///Configuration for collecting the customer's billing address.
    pub billing_address_collection: String,
    ///When set, provides configuration to gather active consent from customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_collection: Option<serde_json::Value>,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///Collect additional information from your customer using custom fields. Up to 2 fields are supported.
    pub custom_fields: Vec<PaymentLinksResourceCustomFields>,
    ///
    pub custom_text: PaymentLinksResourceCustomText,
    ///Configuration for Customer creation during checkout.
    pub customer_creation: String,
    ///Unique identifier for the object.
    pub id: String,
    ///The custom message to be displayed to a customer when a payment link is no longer active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inactive_message: Option<String>,
    ///Configuration for creating invoice for payment mode payment links.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_creation: Option<serde_json::Value>,
    ///The line items representing what is being sold.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<PaymentLinksResourceListLineItems>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The account on behalf of which to charge. See the [Connect documentation](https://support.stripe.com/questions/sending-invoices-on-behalf-of-connected-accounts) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<serde_json::Value>,
    ///Indicates the parameters to be passed to PaymentIntent creation during checkout.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_data: Option<serde_json::Value>,
    ///Configuration for collecting a payment method during checkout.
    pub payment_method_collection: String,
    ///The list of payment method types that customers can use. When `null`, Stripe will dynamically show relevant payment methods you've enabled in your [payment method settings](https://dashboard.stripe.com/settings/payment_methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<String>>,
    ///
    pub phone_number_collection: PaymentLinksResourcePhoneNumberCollection,
    ///Settings that restrict the usage of a payment link.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<serde_json::Value>,
    ///Configuration for collecting the customer's shipping address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<serde_json::Value>,
    ///The shipping rate options applied to the session.
    pub shipping_options: Vec<PaymentLinksResourceShippingOption>,
    ///Indicates the type of transaction being performed which customizes relevant text on the page, such as the submit button.
    pub submit_type: String,
    ///When creating a subscription, the specified configuration data will be used. There must be at least one line item with a recurring price to use `subscription_data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<serde_json::Value>,
    ///
    pub tax_id_collection: PaymentLinksResourceTaxIdCollection,
    ///The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<serde_json::Value>,
    ///The public URL that can be shared with customers.
    pub url: String,
}
impl std::fmt::Display for PaymentLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}