
use serde::{Serialize, Deserialize};
use super::{
    PaymentPagesCheckoutSessionCustomFieldsDropdown,
    PaymentPagesCheckoutSessionCustomFieldsLabel,
    PaymentPagesCheckoutSessionCustomFieldsNumeric,
    PaymentPagesCheckoutSessionCustomFieldsText,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentPagesCheckoutSessionCustomFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<PaymentPagesCheckoutSessionCustomFieldsDropdown>,
    pub key: String,
    pub label: PaymentPagesCheckoutSessionCustomFieldsLabel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric: Option<PaymentPagesCheckoutSessionCustomFieldsNumeric>,
    pub optional: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<PaymentPagesCheckoutSessionCustomFieldsText>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionCustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}