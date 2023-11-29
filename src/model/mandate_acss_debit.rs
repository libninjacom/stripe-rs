
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct MandateAcssDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,
    pub payment_schedule: String,
    pub transaction_type: String,
}
impl std::fmt::Display for MandateAcssDebit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}