
use serde::{Serialize, Deserialize};
use super::Subscription;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionsSubscriptionList {
    pub data: Vec<Subscription>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for SubscriptionsSubscriptionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}