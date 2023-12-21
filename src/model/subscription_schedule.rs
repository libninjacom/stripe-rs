use serde::{Serialize, Deserialize};
use super::{
    SubscriptionSchedulePhaseConfiguration, SubscriptionSchedulesResourceDefaultSettings,
};
/**A subscription schedule allows you to create and manage the lifecycle of a subscription by predefining expected changes.

Related guide: [Subscription schedules](https://stripe.com/docs/billing/subscriptions/subscription-schedules)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionSchedule {
    ///ID of the Connect Application that created the schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<serde_json::Value>,
    ///Time at which the subscription schedule was canceled. Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,
    ///Time at which the subscription schedule was completed. Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Object representing the start and end dates for the current phase of the subscription schedule, if it is `active`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_phase: Option<serde_json::Value>,
    ///ID of the customer who owns the subscription schedule.
    pub customer: serde_json::Value,
    ///
    pub default_settings: SubscriptionSchedulesResourceDefaultSettings,
    ///Behavior of the subscription schedule and underlying subscription when it ends. Possible values are `release` or `cancel` with the default being `release`. `release` will end the subscription schedule and keep the underlying subscription running.`cancel` will end the subscription schedule and cancel the underlying subscription.
    pub end_behavior: String,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Configuration for the subscription schedule's phases.
    pub phases: Vec<SubscriptionSchedulePhaseConfiguration>,
    ///Time at which the subscription schedule was released. Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_at: Option<i64>,
    ///ID of the subscription once managed by the subscription schedule (if it is released).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_subscription: Option<String>,
    ///The present status of the subscription schedule. Possible values are `not_started`, `active`, `completed`, `released`, and `canceled`. You can read more about the different states in our [behavior guide](https://stripe.com/docs/billing/subscriptions/subscription-schedules).
    pub status: String,
    ///ID of the subscription managed by the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<serde_json::Value>,
    ///ID of the test clock this subscription schedule belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<serde_json::Value>,
}
impl std::fmt::Display for SubscriptionSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}