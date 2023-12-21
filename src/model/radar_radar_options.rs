use serde::{Serialize, Deserialize};
///Options to configure Radar. See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RadarRadarOptions {
    ///A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}
impl std::fmt::Display for RadarRadarOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}