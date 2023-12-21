use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountRequirementsError {
    ///The code for the type of error.
    pub code: String,
    ///An informative message that indicates the error type and provides additional details about the error.
    pub reason: String,
    ///The specific user onboarding requirement field (in the requirements hash) that needs to be resolved.
    pub requirement: String,
}
impl std::fmt::Display for AccountRequirementsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}