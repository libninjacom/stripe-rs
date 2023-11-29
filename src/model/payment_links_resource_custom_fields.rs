
use serde::{Serialize, Deserialize};
use super::{
    PaymentLinksResourceCustomFieldsDropdown, PaymentLinksResourceCustomFieldsLabel,
    PaymentLinksResourceCustomFieldsNumeric, PaymentLinksResourceCustomFieldsText,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaymentLinksResourceCustomFields {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropdown: Option<PaymentLinksResourceCustomFieldsDropdown>,
    pub key: String,
    pub label: PaymentLinksResourceCustomFieldsLabel,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub numeric: Option<PaymentLinksResourceCustomFieldsNumeric>,
    pub optional: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<PaymentLinksResourceCustomFieldsText>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for PaymentLinksResourceCustomFields {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}