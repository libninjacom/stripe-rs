use serde::{Serialize, Deserialize};
use super::PersonAdditionalTosAcceptance;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersonAdditionalTosAcceptances {
    ///
    pub account: PersonAdditionalTosAcceptance,
}
impl std::fmt::Display for PersonAdditionalTosAcceptances {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}