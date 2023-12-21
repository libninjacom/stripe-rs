use serde::{Serialize, Deserialize};
use super::NotificationEventData;
/**Events are our way of letting you know when something interesting happens in
your account. When an interesting event occurs, we create a new `Event`
object. For example, when a charge succeeds, we create a `charge.succeeded`
event, and when an invoice payment attempt fails, we create an
`invoice.payment_failed` event. Certain API requests might create multiple
events. For example, if you create a new subscription for a
customer, you receive both a `customer.subscription.created` event and a
`charge.succeeded` event.

Events occur when the state of another API resource changes. The event's data
field embeds the resource's state at the time of the change. For
example, a `charge.succeeded` event contains a charge, and an
`invoice.payment_failed` event contains an invoice.

As with other API resources, you can use endpoints to retrieve an
[individual event](https://stripe.com/docs/api#retrieve_event) or a [list of events](https://stripe.com/docs/api#list_events)
from the API. We also have a separate
[webhooks](http://en.wikipedia.org/wiki/Webhook) system for sending the
`Event` objects directly to an endpoint on your server. You can manage
webhooks in your
[account settings](https://dashboard.stripe.com/account/webhooks). Learn how
to [listen for events](https://stripe.com/docs/webhooks)
so that your integration can automatically trigger reactions.

When using [Connect](https://stripe.com/docs/connect), you can also receive event notifications
that occur in connected accounts. For these events, there's an
additional `account` attribute in the received `Event` object.

We only guarantee access to events through the [Retrieve Event API](https://stripe.com/docs/api#retrieve_event)
for 30 days.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Event {
    ///The connected account that originates the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,
    ///The Stripe API version used to render `data`. This property is populated only for events on or after October 31, 2014.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub api_version: Option<String>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///
    pub data: NotificationEventData,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Number of webhooks that haven't been successfully delivered (for example, to return a 20x response) to the URLs you specify.
    pub pending_webhooks: i64,
    ///Information on the API request that triggers the event.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request: Option<serde_json::Value>,
    ///Description of the event (for example, `invoice.created` or `charge.refunded`).
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for Event {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}