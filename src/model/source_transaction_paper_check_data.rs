
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceTransactionPaperCheckData {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub available_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoices: Option<String>,
}
impl std::fmt::Display for SourceTransactionPaperCheckData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}