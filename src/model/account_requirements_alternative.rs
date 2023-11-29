
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountRequirementsAlternative {
    pub alternative_fields_due: Vec<String>,
    pub original_fields_due: Vec<String>,
}
impl std::fmt::Display for AccountRequirementsAlternative {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}