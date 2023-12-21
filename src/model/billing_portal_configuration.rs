use serde::{Serialize, Deserialize};
use super::{PortalBusinessProfile, PortalFeatures, PortalLoginPage};
///A portal configuration describes the functionality and behavior of a portal session.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BillingPortalConfiguration {
    ///Whether the configuration is active and can be used to create portal sessions.
    pub active: bool,
    ///ID of the Connect Application that created the configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<serde_json::Value>,
    ///
    pub business_profile: PortalBusinessProfile,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///The default URL to redirect customers to when they click on the portal's link to return to your website. This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_return_url: Option<String>,
    ///
    pub features: PortalFeatures,
    ///Unique identifier for the object.
    pub id: String,
    ///Whether the configuration is the default. If `true`, this configuration can be managed in the Dashboard and portal sessions will use this configuration unless it is overriden when creating the session.
    pub is_default: bool,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///
    pub login_page: PortalLoginPage,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Time at which the object was last updated. Measured in seconds since the Unix epoch.
    pub updated: i64,
}
impl std::fmt::Display for BillingPortalConfiguration {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}