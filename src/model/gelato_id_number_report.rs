use serde::{Serialize, Deserialize};
///Result from an id_number check
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GelatoIdNumberReport {
    ///Date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<serde_json::Value>,
    ///Details on the verification error. Present when status is `unverified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    ///First name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    ///ID number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<String>,
    ///Type of ID number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number_type: Option<String>,
    ///Last name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    ///Status of this `id_number` check.
    pub status: String,
}
impl std::fmt::Display for GelatoIdNumberReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}