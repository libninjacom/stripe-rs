
use serde::{Serialize, Deserialize};
use super::ClimateRemovalsLocation;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ClimateSupplier {
    pub id: String,
    pub info_url: String,
    pub livemode: bool,
    pub locations: Vec<ClimateRemovalsLocation>,
    pub name: String,
    pub object: String,
    pub removal_pathway: String,
}
impl std::fmt::Display for ClimateSupplier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}