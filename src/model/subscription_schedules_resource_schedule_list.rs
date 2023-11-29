
use serde::{Serialize, Deserialize};
use super::SubscriptionSchedule;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionSchedulesResourceScheduleList {
    pub data: Vec<SubscriptionSchedule>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for SubscriptionSchedulesResourceScheduleList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}