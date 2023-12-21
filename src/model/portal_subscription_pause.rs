use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortalSubscriptionPause {
    ///Whether the feature is enabled.
    pub enabled: bool,
}
impl std::fmt::Display for PortalSubscriptionPause {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}