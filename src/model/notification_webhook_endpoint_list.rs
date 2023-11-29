
use serde::{Serialize, Deserialize};
use super::WebhookEndpoint;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct NotificationWebhookEndpointList {
    pub data: Vec<WebhookEndpoint>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for NotificationWebhookEndpointList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}