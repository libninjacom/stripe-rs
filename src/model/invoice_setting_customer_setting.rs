
use serde::{Serialize, Deserialize};
use super::InvoiceSettingCustomField;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceSettingCustomerSetting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<InvoiceSettingCustomField>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<serde_json::Value>,
}
impl std::fmt::Display for InvoiceSettingCustomerSetting {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}