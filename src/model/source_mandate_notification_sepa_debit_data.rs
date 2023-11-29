
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceMandateNotificationSepaDebitData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creditor_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_reference: Option<String>,
}
impl std::fmt::Display for SourceMandateNotificationSepaDebitData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}