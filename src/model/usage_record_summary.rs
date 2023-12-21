use serde::{Serialize, Deserialize};
use super::Period;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct UsageRecordSummary {
    ///Unique identifier for the object.
    pub id: String,
    ///The invoice in which this usage period has been billed for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///
    pub period: Period,
    ///The ID of the subscription item this summary is describing.
    pub subscription_item: String,
    ///The total usage within this usage period.
    pub total_usage: i64,
}
impl std::fmt::Display for UsageRecordSummary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}