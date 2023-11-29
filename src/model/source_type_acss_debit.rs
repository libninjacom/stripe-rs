
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceTypeAcssDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_address_city: Option<String>,
    #[serde(rename = "bank_address_line_1")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_address_line1: Option<String>,
    #[serde(rename = "bank_address_line_2")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_address_line2: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_address_postal_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}
impl std::fmt::Display for SourceTypeAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}