
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RadarValueListItem {
    pub created: i64,
    pub created_by: String,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub value: String,
    pub value_list: String,
}
impl std::fmt::Display for RadarValueListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}