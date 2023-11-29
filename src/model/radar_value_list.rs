
use serde::{Serialize, Deserialize};
use super::RadarListListItemList;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadarValueList {
    pub alias: String,
    pub created: i64,
    pub created_by: String,
    pub id: String,
    pub item_type: String,
    pub list_items: RadarListListItemList,
    pub livemode: bool,
    pub metadata: serde_json::Value,
    pub name: String,
    pub object: String,
}
impl std::fmt::Display for RadarValueList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}