use serde::{Serialize, Deserialize};
use super::Customer;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetCustomersSearchResponse {
    pub data: Vec<Customer>,
    pub has_more: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_page: Option<String>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The total number of objects that match the query, only accurate up to 10,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<i64>,
    pub url: String,
}
impl std::fmt::Display for GetCustomersSearchResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}