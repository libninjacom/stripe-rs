use serde::{Serialize, Deserialize};
///[Tax codes](https://stripe.com/docs/tax/tax-categories) classify goods and services for tax purposes.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxCode {
    ///A detailed description of which types of products the tax code represents.
    pub description: String,
    ///Unique identifier for the object.
    pub id: String,
    ///A short name for the tax code.
    pub name: String,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for TaxCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}