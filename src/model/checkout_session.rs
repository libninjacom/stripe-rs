
use serde::{Serialize, Deserialize};
use super::{
    PaymentPagesCheckoutSessionAutomaticTax, PaymentPagesCheckoutSessionCustomFields,
    PaymentPagesCheckoutSessionCustomText, PaymentPagesCheckoutSessionListLineItems,
    PaymentPagesCheckoutSessionPhoneNumberCollection,
    PaymentPagesCheckoutSessionShippingOption, PaymentPagesCheckoutSessionTaxIdCollection,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckoutSession {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub after_expiration: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_promotion_codes: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_subtotal: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_total: Option<i64>,
    pub automatic_tax: PaymentPagesCheckoutSessionAutomaticTax,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address_collection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_reference_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_collection: Option<serde_json::Value>,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_conversion: Option<serde_json::Value>,
    pub custom_fields: Vec<PaymentPagesCheckoutSessionCustomFields>,
    pub custom_text: PaymentPagesCheckoutSessionCustomText,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_creation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email: Option<String>,
    pub expires_at: i64,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_creation: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<PaymentPagesCheckoutSessionListLineItems>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub mode: String,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_link: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_collection: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_configuration_details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<serde_json::Value>,
    pub payment_method_types: Vec<String>,
    pub payment_status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number_collection: Option<
        PaymentPagesCheckoutSessionPhoneNumberCollection,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recovered_from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect_on_completion: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_intent: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<serde_json::Value>,
    pub shipping_options: Vec<PaymentPagesCheckoutSessionShippingOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success_url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_collection: Option<PaymentPagesCheckoutSessionTaxIdCollection>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ui_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl std::fmt::Display for CheckoutSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}