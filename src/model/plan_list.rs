
use serde::{Serialize, Deserialize};
use super::Plan;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlanList {
    pub data: Vec<Plan>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for PlanList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}