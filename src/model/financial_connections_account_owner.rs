use serde::{Serialize, Deserialize};
///Describes an owner of an account.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FinancialConnectionsAccountOwner {
    ///The email address of the owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    ///Unique identifier for the object.
    pub id: String,
    ///The full name of the owner.
    pub name: String,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The ownership object that this owner belongs to.
    pub ownership: String,
    ///The raw phone number of the owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    ///The raw physical address of the owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub raw_address: Option<String>,
    ///The timestamp of the refresh that updated this owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refreshed_at: Option<i64>,
}
impl std::fmt::Display for FinancialConnectionsAccountOwner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}