use serde::{Serialize, Deserialize};
///Settings for automatic tax lookup for this invoice.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AutomaticTaxParam {
    pub enabled: bool,
}
impl std::fmt::Display for AutomaticTaxParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}