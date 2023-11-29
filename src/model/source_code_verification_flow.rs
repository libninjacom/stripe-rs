
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceCodeVerificationFlow {
    pub attempts_remaining: i64,
    pub status: String,
}
impl std::fmt::Display for SourceCodeVerificationFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}