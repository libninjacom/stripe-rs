
use serde::{Serialize, Deserialize};
use super::{
    AccountCapabilities, AccountFutureRequirements, AccountRequirements,
    AccountTosAcceptance, AccountUnificationAccountController, ExternalAccountList,
    LegalEntityCompany, Person,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Account {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<AccountCapabilities>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<LegalEntityCompany>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<AccountUnificationAccountController>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_submitted: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_accounts: Option<ExternalAccountList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<AccountFutureRequirements>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<Person>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts_enabled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<AccountRequirements>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<AccountTosAcceptance>,
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}