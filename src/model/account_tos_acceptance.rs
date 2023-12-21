use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountTosAcceptance {
    ///The Unix timestamp marking when the account representative accepted their service agreement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<i64>,
    ///The IP address from which the account representative accepted their service agreement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,
    ///The user's service agreement type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_agreement: Option<String>,
    ///The user agent of the browser from which the account representative accepted their service agreement
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
impl std::fmt::Display for AccountTosAcceptance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}