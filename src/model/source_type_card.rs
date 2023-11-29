
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceTypeCard {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1_check: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip_check: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc_check: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dynamic_last4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tokenization_method: Option<String>,
}
impl std::fmt::Display for SourceTypeCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}