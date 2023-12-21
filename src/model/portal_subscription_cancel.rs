use serde::{Serialize, Deserialize};
use super::PortalSubscriptionCancellationReason;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortalSubscriptionCancel {
    ///
    pub cancellation_reason: PortalSubscriptionCancellationReason,
    ///Whether the feature is enabled.
    pub enabled: bool,
    ///Whether to cancel subscriptions immediately or at the end of the billing period.
    pub mode: String,
    ///Whether to create prorations when canceling subscriptions. Possible values are `none` and `create_prorations`.
    pub proration_behavior: String,
}
impl std::fmt::Display for PortalSubscriptionCancel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}