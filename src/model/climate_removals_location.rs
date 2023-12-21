use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClimateRemovalsLocation {
    ///The city where the supplier is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,
    ///Two-letter ISO code representing the country where the supplier is located.
    pub country: String,
    ///The geographic latitude where the supplier is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub latitude: Option<f64>,
    ///The geographic longitude where the supplier is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longitude: Option<f64>,
    ///The state/county/province/region where the supplier is located.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region: Option<String>,
}
impl std::fmt::Display for ClimateRemovalsLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}