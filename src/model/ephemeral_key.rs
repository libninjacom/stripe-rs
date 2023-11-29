
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct EphemeralKey {
    pub created: i64,
    pub expires: i64,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
}
impl std::fmt::Display for EphemeralKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}