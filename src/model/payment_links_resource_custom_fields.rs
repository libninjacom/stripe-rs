use serde::{Serialize, Deserialize};
use super::{
    PaymentLinksResourceCustomFieldsDropdown, PaymentLinksResourceCustomFieldsLabel,
    PaymentLinksResourceCustomFieldsNumeric, PaymentLinksResourceCustomFieldsText,
};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLinksResourceCustomFields {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<PaymentLinksResourceCustomFieldsDropdown>,
    ///String of your choice that your integration can use to reconcile this field. Must be unique to this field, alphanumeric, and up to 200 characters.
    pub key: String,
    ///
    pub label: PaymentLinksResourceCustomFieldsLabel,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric: Option<PaymentLinksResourceCustomFieldsNumeric>,
    ///Whether the customer is required to complete the field before completing the Checkout Session. Defaults to `false`.
    pub optional: bool,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<PaymentLinksResourceCustomFieldsText>,
    ///The type of the field.
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for PaymentLinksResourceCustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}