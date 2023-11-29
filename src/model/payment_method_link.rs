
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodLink {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}
impl std::fmt::Display for PaymentMethodLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}