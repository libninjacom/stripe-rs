
use serde::{Serialize, Deserialize};
use super::{
    PaymentLinksResourceAfterCompletion, PaymentLinksResourceAutomaticTax,
    PaymentLinksResourceCustomFields, PaymentLinksResourceCustomText,
    PaymentLinksResourceListLineItems, PaymentLinksResourcePhoneNumberCollection,
    PaymentLinksResourceShippingOption, PaymentLinksResourceTaxIdCollection,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentLink {
    pub active: bool,
    pub after_completion: PaymentLinksResourceAfterCompletion,
    pub allow_promotion_codes: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,
    pub automatic_tax: PaymentLinksResourceAutomaticTax,
    pub billing_address_collection: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub consent_collection: Option<serde_json::Value>,
    pub currency: String,
    pub custom_fields: Vec<PaymentLinksResourceCustomFields>,
    pub custom_text: PaymentLinksResourceCustomText,
    pub customer_creation: String,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_creation: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<PaymentLinksResourceListLineItems>,
    pub livemode: bool,
    pub metadata: serde_json::Value,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent_data: Option<serde_json::Value>,
    pub payment_method_collection: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<String>>,
    pub phone_number_collection: PaymentLinksResourcePhoneNumberCollection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address_collection: Option<serde_json::Value>,
    pub shipping_options: Vec<PaymentLinksResourceShippingOption>,
    pub submit_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<serde_json::Value>,
    pub tax_id_collection: PaymentLinksResourceTaxIdCollection,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<serde_json::Value>,
    pub url: String,
}
impl std::fmt::Display for PaymentLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}