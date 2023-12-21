use serde::{Serialize, Deserialize};
/**The Billing customer portal is a Stripe-hosted UI for subscription and
billing management.

A portal configuration describes the functionality and features that you
want to provide to your customers through the portal.

A portal session describes the instantiation of the customer portal for
a particular customer. By visiting the session's URL, the customer
can manage their subscriptions and billing details. For security reasons,
sessions are short-lived and will expire if the customer does not visit the URL.
Create sessions on-demand when customers intend to manage their subscriptions
and billing details.

Learn more in the [integration guide](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal).*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BillingPortalSession {
    ///The configuration used by this session, describing the features available.
    pub configuration: serde_json::Value,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///The ID of the customer for this session.
    pub customer: String,
    ///Information about a specific flow for the customer to go through. See the [docs](https://stripe.com/docs/customer-management/portal-deep-links) to learn more about using customer portal deep links and flows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<serde_json::Value>,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///The IETF language tag of the locale Customer Portal is displayed in. If blank or auto, the customer’s `preferred_locales` or browser’s locale is used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub locale: Option<String>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The account for which the session was created on behalf of. When specified, only subscriptions and invoices with this `on_behalf_of` account appear in the portal. For more information, see the [docs](https://stripe.com/docs/connect/separate-charges-and-transfers#on-behalf-of). Use the [Accounts API](https://stripe.com/docs/api/accounts/object#account_object-settings-branding) to modify the `on_behalf_of` account's branding settings, which the portal displays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,
    ///The URL to redirect customers to when they click on the portal's link to return to your website.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
    ///The short-lived URL of the session that gives customers access to the customer portal.
    pub url: String,
}
impl std::fmt::Display for BillingPortalSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}