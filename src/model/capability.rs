use serde::{Serialize, Deserialize};
use super::{AccountCapabilityFutureRequirements, AccountCapabilityRequirements};
/**This is an object representing a capability for a Stripe account.

Related guide: [Account capabilities](https://stripe.com/docs/connect/account-capabilities)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Capability {
    ///The account for which the capability enables functionality.
    pub account: serde_json::Value,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<AccountCapabilityFutureRequirements>,
    ///The identifier for the capability.
    pub id: String,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Whether the capability has been requested.
    pub requested: bool,
    ///Time at which the capability was requested. Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_at: Option<i64>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<AccountCapabilityRequirements>,
    ///The status of the capability. Can be `active`, `inactive`, `pending`, or `unrequested`.
    pub status: String,
}
impl std::fmt::Display for Capability {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}