
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxProductResourceJurisdiction {
    pub country: String,
    pub display_name: String,
    pub level: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}
impl std::fmt::Display for TaxProductResourceJurisdiction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}