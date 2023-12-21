use serde::{Serialize, Deserialize};
use super::{
    PaymentPagesCheckoutSessionCustomFieldsDropdown,
    PaymentPagesCheckoutSessionCustomFieldsLabel,
    PaymentPagesCheckoutSessionCustomFieldsNumeric,
    PaymentPagesCheckoutSessionCustomFieldsText,
};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentPagesCheckoutSessionCustomFields {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<PaymentPagesCheckoutSessionCustomFieldsDropdown>,
    ///String of your choice that your integration can use to reconcile this field. Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: String,
    ///
    pub label: PaymentPagesCheckoutSessionCustomFieldsLabel,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric: Option<PaymentPagesCheckoutSessionCustomFieldsNumeric>,
    ///Whether the customer is required to complete the field before completing the Checkout Session. Defaults to `false`.
    pub optional: bool,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<PaymentPagesCheckoutSessionCustomFieldsText>,
    ///The type of the field.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionCustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}