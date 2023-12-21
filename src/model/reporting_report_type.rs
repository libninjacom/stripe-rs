use serde::{Serialize, Deserialize};
/**The Report Type resource corresponds to a particular type of report, such as
the "Activity summary" or "Itemized payouts" reports. These objects are
identified by an ID belonging to a set of enumerated values. See
[API Access to Reports documentation](https://stripe.com/docs/reporting/statements/api)
for those Report Type IDs, along with required and optional parameters.

Note that certain report types can only be run based on your live-mode data (not test-mode
data), and will error when queried without a [live-mode API key](https://stripe.com/docs/keys#test-live-modes).*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ReportingReportType {
    ///Most recent time for which this Report Type is available. Measured in seconds since the Unix epoch.
    pub data_available_end: i64,
    ///Earliest time for which this Report Type is available. Measured in seconds since the Unix epoch.
    pub data_available_start: i64,
    ///List of column names that are included by default when this Report Type gets run. (If the Report Type doesn't support the `columns` parameter, this will be null.)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_columns: Option<Vec<String>>,
    ///The [ID of the Report Type](https://stripe.com/docs/reporting/statements/api#available-report-types), such as `balance.summary.1`.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Human-readable name of the Report Type
    pub name: String,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///When this Report Type was latest updated. Measured in seconds since the Unix epoch.
    pub updated: i64,
    ///Version of the Report Type. Different versions report with the same ID will have the same purpose, but may take different run parameters or have different result schemas.
    pub version: i64,
}
impl std::fmt::Display for ReportingReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}