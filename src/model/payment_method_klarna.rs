use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodKlarna {
    ///The customer's date of birth, if provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentMethodKlarna {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}