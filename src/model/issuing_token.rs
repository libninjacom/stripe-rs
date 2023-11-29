
use serde::{Serialize, Deserialize};
use super::IssuingNetworkTokenNetworkData;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IssuingToken {
    pub card: serde_json::Value,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_fingerprint: Option<String>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    pub livemode: bool,
    pub network: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_data: Option<IssuingNetworkTokenNetworkData>,
    pub network_updated_at: i64,
    pub object: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_provider: Option<String>,
}
impl std::fmt::Display for IssuingToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}