
use serde::{Serialize, Deserialize};
use super::TestHelpersTestClock;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BillingClocksResourceBillingClockList {
    pub data: Vec<TestHelpersTestClock>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for BillingClocksResourceBillingClockList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}