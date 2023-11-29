
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageRecord {
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub quantity: i64,
    pub subscription_item: String,
    pub timestamp: i64,
}
impl std::fmt::Display for UsageRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}