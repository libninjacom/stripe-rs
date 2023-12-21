use serde::{Serialize, Deserialize};
/**Usage records allow you to report customer usage and metrics to Stripe for
metered billing of subscription prices.

Related guide: [Metered billing](https://stripe.com/docs/billing/subscriptions/metered-billing)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageRecord {
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The usage quantity for the specified date.
    pub quantity: i64,
    ///The ID of the subscription item this usage record contains data for.
    pub subscription_item: String,
    ///The timestamp when this usage occurred.
    pub timestamp: i64,
}
impl std::fmt::Display for UsageRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}