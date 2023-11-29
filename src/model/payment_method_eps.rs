
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodEps {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<String>,
}
impl std::fmt::Display for PaymentMethodEps {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}