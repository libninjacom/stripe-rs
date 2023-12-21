use serde::{Serialize, Deserialize};
use super::ConnectEmbeddedPaymentsFeatures;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectEmbeddedPaymentsConfig {
    ///Whether the embedded component is enabled.
    pub enabled: bool,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<ConnectEmbeddedPaymentsFeatures>,
}
impl std::fmt::Display for ConnectEmbeddedPaymentsConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}