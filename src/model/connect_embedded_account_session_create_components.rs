
use serde::{Serialize, Deserialize};
use super::ConnectEmbeddedBaseConfigClaim;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectEmbeddedAccountSessionCreateComponents {
    pub account_onboarding: ConnectEmbeddedBaseConfigClaim,
}
impl std::fmt::Display for ConnectEmbeddedAccountSessionCreateComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}