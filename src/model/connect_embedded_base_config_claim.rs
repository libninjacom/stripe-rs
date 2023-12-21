use serde::{Serialize, Deserialize};
use super::ConnectEmbeddedBaseFeatures;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectEmbeddedBaseConfigClaim {
    ///Whether the embedded component is enabled.
    pub enabled: bool,
    ///
    pub features: ConnectEmbeddedBaseFeatures,
}
impl std::fmt::Display for ConnectEmbeddedBaseConfigClaim {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}