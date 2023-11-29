
use serde::{Serialize, Deserialize};
use super::RadarValueList;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RadarListListList {
    pub data: Vec<RadarValueList>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for RadarListListList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}