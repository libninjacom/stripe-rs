use serde::{Serialize, Deserialize};
use super::{
    ConnectEmbeddedBaseConfigClaim, ConnectEmbeddedPaymentsConfig,
    ConnectEmbeddedPayoutsConfig,
};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConnectEmbeddedAccountSessionCreateComponents {
    ///
    pub account_onboarding: ConnectEmbeddedBaseConfigClaim,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_details: Option<ConnectEmbeddedPaymentsConfig>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payments: Option<ConnectEmbeddedPaymentsConfig>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<ConnectEmbeddedPayoutsConfig>,
}
impl std::fmt::Display for ConnectEmbeddedAccountSessionCreateComponents {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}