
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentLinksResourceCustomFieldsDropdownOption {
    pub label: String,
    pub value: String,
}
impl std::fmt::Display for PaymentLinksResourceCustomFieldsDropdownOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}