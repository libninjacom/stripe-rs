
use serde::{Serialize, Deserialize};
use super::PaymentPagesCheckoutSessionCustomFieldsOption;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentPagesCheckoutSessionCustomFieldsDropdown {
    pub options: Vec<PaymentPagesCheckoutSessionCustomFieldsOption>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionCustomFieldsDropdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}