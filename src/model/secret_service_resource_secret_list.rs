
use serde::{Serialize, Deserialize};
use super::AppsSecret;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SecretServiceResourceSecretList {
    pub data: Vec<AppsSecret>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for SecretServiceResourceSecretList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}