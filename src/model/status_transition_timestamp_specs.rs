use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct StatusTransitionTimestampSpecs {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<serde_json::Value>,
}
impl std::fmt::Display for StatusTransitionTimestampSpecs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}