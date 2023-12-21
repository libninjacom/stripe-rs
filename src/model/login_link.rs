use serde::{Serialize, Deserialize};
///Login Links are single-use login link for an Express account to access their Stripe dashboard.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LoginLink {
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The URL for the login link.
    pub url: String,
}
impl std::fmt::Display for LoginLink {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}