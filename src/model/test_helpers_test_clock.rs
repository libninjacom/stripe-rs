
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestHelpersTestClock {
    pub created: i64,
    pub deletes_after: i64,
    pub frozen_time: i64,
    pub id: String,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    pub object: String,
    pub status: String,
}
impl std::fmt::Display for TestHelpersTestClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}