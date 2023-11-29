
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeletedTerminalConfiguration {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}
impl std::fmt::Display for DeletedTerminalConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}