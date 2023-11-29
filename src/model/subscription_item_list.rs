
use serde::{Serialize, Deserialize};
use super::SubscriptionItem;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionItemList {
    pub data: Vec<SubscriptionItem>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for SubscriptionItemList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}