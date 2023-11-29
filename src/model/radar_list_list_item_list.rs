
use serde::{Serialize, Deserialize};
use super::RadarValueListItem;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RadarListListItemList {
    pub data: Vec<RadarValueListItem>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for RadarListListItemList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}