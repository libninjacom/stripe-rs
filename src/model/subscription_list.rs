
use serde::{Serialize, Deserialize};
use super::Subscription;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionList {
    pub data: Vec<Subscription>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for SubscriptionList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}