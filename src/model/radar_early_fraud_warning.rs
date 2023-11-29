
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarEarlyFraudWarning {
    pub actionable: bool,
    pub charge: serde_json::Value,
    pub created: i64,
    pub fraud_type: String,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<serde_json::Value>,
}
impl std::fmt::Display for RadarEarlyFraudWarning {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}