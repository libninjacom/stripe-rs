
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<serde_json::Value>,
}
impl std::fmt::Display for TaxParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}