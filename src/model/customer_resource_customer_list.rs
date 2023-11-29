
use serde::{Serialize, Deserialize};
use super::Customer;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CustomerResourceCustomerList {
    pub data: Vec<Customer>,
    pub has_more: bool,
    pub object: String,
    pub url: String,
}
impl std::fmt::Display for CustomerResourceCustomerList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}