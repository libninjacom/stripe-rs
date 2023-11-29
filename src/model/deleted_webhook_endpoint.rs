
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DeletedWebhookEndpoint {
    pub deleted: bool,
    pub id: String,
    pub object: String,
}
impl std::fmt::Display for DeletedWebhookEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}