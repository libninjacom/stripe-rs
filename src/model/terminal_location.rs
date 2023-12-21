use serde::{Serialize, Deserialize};
use super::Address;
/**A Location represents a grouping of readers.

Related guide: [Fleet management](https://stripe.com/docs/terminal/fleet/locations)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalLocation {
    ///
    pub address: Address,
    ///The ID of a configuration that will be used to customize all readers in this location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<String>,
    ///The display name of the location.
    pub display_name: String,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for TerminalLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}