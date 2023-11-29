
use serde::{Serialize, Deserialize};
use super::{
    SubscriptionSchedulePhaseConfiguration, SubscriptionSchedulesResourceDefaultSettings,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubscriptionSchedule {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<i64>,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_phase: Option<serde_json::Value>,
    pub customer: serde_json::Value,
    pub default_settings: SubscriptionSchedulesResourceDefaultSettings,
    pub end_behavior: String,
    pub id: String,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub object: String,
    pub phases: Vec<SubscriptionSchedulePhaseConfiguration>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_subscription: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<serde_json::Value>,
}
impl std::fmt::Display for SubscriptionSchedule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}