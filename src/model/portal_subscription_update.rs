use serde::{Serialize, Deserialize};
use super::PortalSubscriptionUpdateProduct;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PortalSubscriptionUpdate {
    ///The types of subscription updates that are supported for items listed in the `products` attribute. When empty, subscriptions are not updateable.
    pub default_allowed_updates: Vec<String>,
    ///Whether the feature is enabled.
    pub enabled: bool,
    ///The list of up to 10 products that support subscription updates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<PortalSubscriptionUpdateProduct>>,
    ///Determines how to handle prorations resulting from subscription updates. Valid values are `none`, `create_prorations`, and `always_invoice`.
    pub proration_behavior: String,
}
impl std::fmt::Display for PortalSubscriptionUpdate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}