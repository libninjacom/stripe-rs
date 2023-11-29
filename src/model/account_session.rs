
use serde::{Serialize, Deserialize};
use super::ConnectEmbeddedAccountSessionCreateComponents;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccountSession {
    pub account: String,
    pub client_secret: String,
    pub components: ConnectEmbeddedAccountSessionCreateComponents,
    pub expires_at: i64,
    pub livemode: bool,
    pub object: String,
}
impl std::fmt::Display for AccountSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}