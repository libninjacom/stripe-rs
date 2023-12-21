use serde::{Serialize, Deserialize};
/**You can configure [webhook endpoints](https://stripe.com/docs/webhooks/) via the API to be
notified about events that happen in your Stripe account or connected
accounts.

Most users configure webhooks from [the dashboard](https://dashboard.stripe.com/webhooks), which provides a user interface for registering and testing your webhook endpoints.

Related guide: [Setting up webhooks](https://stripe.com/docs/webhooks/configure)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct WebhookEndpoint {
    ///The API version events are rendered as for this webhook endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    ///The ID of the associated Connect application.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<String>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///An optional description of what the webhook is used for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    ///The list of events to enable for this endpoint. `['*']` indicates that all events are enabled, except those that require explicit selection.
    pub enabled_events: Vec<String>,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The endpoint's secret, used to generate [webhook signatures](https://stripe.com/docs/webhooks/signatures). Only returned at creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secret: Option<String>,
    ///The status of the webhook. It can be `enabled` or `disabled`.
    pub status: String,
    ///The URL of the webhook endpoint.
    pub url: String,
}
impl std::fmt::Display for WebhookEndpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}