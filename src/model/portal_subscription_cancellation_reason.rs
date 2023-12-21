use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortalSubscriptionCancellationReason {
    ///Whether the feature is enabled.
    pub enabled: bool,
    ///Which cancellation reasons will be given as options to the customer.
    pub options: Vec<String>,
}
impl std::fmt::Display for PortalSubscriptionCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}