use serde::{Serialize, Deserialize};
use super::{
    BankConnectionsResourceLinkAccountSessionFilters,
    BankConnectionsResourceLinkedAccountList,
};
///A Financial Connections Session is the secure way to programmatically launch the client-side Stripe.js modal that lets your users link their accounts.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FinancialConnectionsSession {
    ///The account holder for whom accounts are collected in this session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder: Option<serde_json::Value>,
    ///The accounts that were collected as part of this Session.
    pub accounts: BankConnectionsResourceLinkedAccountList,
    ///A value that will be passed to the client to launch the authentication flow.
    pub client_secret: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<BankConnectionsResourceLinkAccountSessionFilters>,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Permissions requested for accounts collected during this session.
    pub permissions: Vec<String>,
    ///Data features requested to be retrieved upon account creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefetch: Option<Vec<String>>,
    ///For webview integrations only. Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}
impl std::fmt::Display for FinancialConnectionsSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}