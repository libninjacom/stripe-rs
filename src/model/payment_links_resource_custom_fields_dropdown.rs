
use serde::{Serialize, Deserialize};
use super::PaymentLinksResourceCustomFieldsDropdownOption;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLinksResourceCustomFieldsDropdown {
    pub options: Vec<PaymentLinksResourceCustomFieldsDropdownOption>,
}
impl std::fmt::Display for PaymentLinksResourceCustomFieldsDropdown {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}