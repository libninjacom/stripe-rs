use serde::{Serialize, Deserialize};
/**Account Links are the means by which a Connect platform grants a connected account permission to access
Stripe-hosted applications, such as Connect Onboarding.

Related guide: [Connect Onboarding](https://stripe.com/docs/connect/custom/hosted-onboarding)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountLink {
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///The timestamp at which this account link will expire.
    pub expires_at: i64,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The URL for the account link.
    pub url: String,
}
impl std::fmt::Display for AccountLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}