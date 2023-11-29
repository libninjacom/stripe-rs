
use serde::{Serialize, Deserialize};
use super::{DataParams, TaxParam};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerDetailsParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<TaxParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<Vec<DataParams>>,
}
impl std::fmt::Display for CustomerDetailsParam {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}