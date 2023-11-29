
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetupAttemptPaymentMethodDetailsCardPresent {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub generated_card: Option<serde_json::Value>,
}
impl std::fmt::Display for SetupAttemptPaymentMethodDetailsCardPresent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}