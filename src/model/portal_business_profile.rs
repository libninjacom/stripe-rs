use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortalBusinessProfile {
    ///The messaging shown to customers in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,
    ///A link to the business’s publicly available privacy policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<String>,
    ///A link to the business’s publicly available terms of service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_url: Option<String>,
}
impl std::fmt::Display for PortalBusinessProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}