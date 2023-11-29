
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountUnificationAccountController {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_controller: Option<bool>,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for AccountUnificationAccountController {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}