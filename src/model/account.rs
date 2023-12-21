use serde::{Serialize, Deserialize};
use super::{
    AccountCapabilities, AccountFutureRequirements, AccountRequirements,
    AccountTosAcceptance, AccountUnificationAccountController, ExternalAccountList,
    LegalEntityCompany, Person,
};
/**This is an object representing a Stripe account. You can retrieve it to see
properties on the account like its current requirements or if the account is
enabled to make live charges or receive payouts.

For Custom accounts, the properties below are always returned. For other accounts, some properties are returned until that
account has started to go through Connect Onboarding. Once you create an [Account Link](https://stripe.com/docs/api/account_links)
for a Standard or Express account, some parameters are no longer returned. These are marked as **Custom Only** or **Custom and Express**
below. Learn about the differences [between accounts](https://stripe.com/docs/connect/accounts).*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Account {
    ///Business information about the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<serde_json::Value>,
    ///The business type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_type: Option<String>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capabilities: Option<AccountCapabilities>,
    ///Whether the account can create live charges.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charges_enabled: Option<bool>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub company: Option<LegalEntityCompany>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub controller: Option<AccountUnificationAccountController>,
    ///The account's country.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    ///Time at which the account was connected. Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<i64>,
    ///Three-letter ISO currency code representing the default currency for the account. This must be a currency that [Stripe supports in the account's country](https://stripe.com/docs/payouts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_currency: Option<String>,
    ///Whether account details have been submitted. Standard accounts cannot receive payouts before this is true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub details_submitted: Option<bool>,
    ///An email address associated with the account. It's not used for authentication and Stripe doesn't market to this field without explicit approval from the platform.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    ///External accounts (bank accounts and debit cards) currently attached to this account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_accounts: Option<ExternalAccountList>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<AccountFutureRequirements>,
    ///Unique identifier for the object.
    pub id: String,
    /**This is an object representing a person associated with a Stripe account.

A platform cannot access a Standard or Express account's persons after the account starts onboarding, such as after generating an account link for the account.
See the [Standard onboarding](https://stripe.com/docs/connect/standard-accounts) or [Express onboarding documentation](https://stripe.com/docs/connect/express-accounts) for information about platform prefilling and account onboarding steps.

Related guide: [Handling identity verification with the API](https://stripe.com/docs/connect/handling-api-verification#person-information)*/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individual: Option<Person>,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///Whether Stripe can send payouts to this account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts_enabled: Option<bool>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<AccountRequirements>,
    ///Options for customizing how the account functions within Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<serde_json::Value>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<AccountTosAcceptance>,
    ///The Stripe account type. Can be `standard`, `express`, or `custom`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}