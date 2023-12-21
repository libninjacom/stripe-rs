use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceSettingCustomField {
    ///The name of the custom field.
    pub name: String,
    ///The value of the custom field.
    pub value: String,
}
impl std::fmt::Display for InvoiceSettingCustomField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}