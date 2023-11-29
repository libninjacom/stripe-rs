
use serde::{Serialize, Deserialize};
use super::CreditNote;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CreditNotesList {
    pub data: Vec<CreditNote>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for CreditNotesList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}