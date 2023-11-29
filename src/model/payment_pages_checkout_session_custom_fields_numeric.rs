
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentPagesCheckoutSessionCustomFieldsNumeric {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_length: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
}
impl std::fmt::Display for PaymentPagesCheckoutSessionCustomFieldsNumeric {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}