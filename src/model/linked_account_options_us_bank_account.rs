
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LinkedAccountOptionsUsBankAccount {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}
impl std::fmt::Display for LinkedAccountOptionsUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}