
use serde::{Serialize, Deserialize};
use super::ProductFeature;
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub active: bool,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_price: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub features: Vec<ProductFeature>,
    pub id: String,
    pub images: Vec<String>,
    pub livemode: bool,
    pub metadata: serde_json::Value,
    pub name: String,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shippable: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_label: Option<String>,
    pub updated: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}
impl std::fmt::Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}