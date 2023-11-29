
use serde::{Serialize, Deserialize};
use super::CountrySpec;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CountrySpecList {
    pub data: Vec<CountrySpec>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for CountrySpecList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}