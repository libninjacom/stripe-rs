
use serde::{Serialize, Deserialize};
use super::PortalSubscriptionCancellationReason;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortalSubscriptionCancel {
    pub cancellation_reason: PortalSubscriptionCancellationReason,
    pub enabled: bool,
    pub mode: String,
    pub proration_behavior: String,
}
impl std::fmt::Display for PortalSubscriptionCancel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}