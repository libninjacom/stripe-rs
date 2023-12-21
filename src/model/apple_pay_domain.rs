use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ApplePayDomain {
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    pub domain_name: String,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for ApplePayDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}