use serde::{Serialize, Deserialize};
use super::SigmaScheduledQueryRunError;
/**If you have [scheduled a Sigma query](https://stripe.com/docs/sigma/scheduled-queries), you'll
receive a `sigma.scheduled_query_run.created` webhook each time the query
runs. The webhook contains a `ScheduledQueryRun` object, which you can use to
retrieve the query results.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScheduledQueryRun {
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///When the query was run, Sigma contained a snapshot of your Stripe data at this time.
    pub data_load_time: i64,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<SigmaScheduledQueryRunError>,
    ///The file object representing the results of the query.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<serde_json::Value>,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Time at which the result expires and is no longer available for download.
    pub result_available_until: i64,
    ///SQL for the query.
    pub sql: String,
    ///The query's execution status, which will be `completed` for successful runs, and `canceled`, `failed`, or `timed_out` otherwise.
    pub status: String,
    ///Title of the query.
    pub title: String,
}
impl std::fmt::Display for ScheduledQueryRun {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}