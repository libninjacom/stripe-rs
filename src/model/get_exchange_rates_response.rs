use serde::{Serialize, Deserialize};
use super::ExchangeRate;
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetExchangeRatesResponse {
    pub data: Vec<ExchangeRate>,
    ///True if this list has another page of items after this one that can be fetched.
    pub has_more: bool,
    ///String representing the object's type. Objects of the same type share the same value. Always has the value `list`.
    pub object: String,
    ///The URL where this list can be accessed.
    pub url: String,
}
impl std::fmt::Display for GetExchangeRatesResponse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}