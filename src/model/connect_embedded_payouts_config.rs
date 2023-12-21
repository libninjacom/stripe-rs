use serde::{Serialize, Deserialize};
use super::ConnectEmbeddedPayoutsFeatures;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectEmbeddedPayoutsConfig {
    ///Whether the embedded component is enabled.
    pub enabled: bool,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<ConnectEmbeddedPayoutsFeatures>,
}
impl std::fmt::Display for ConnectEmbeddedPayoutsConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}