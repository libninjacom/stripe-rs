use serde::{Serialize, Deserialize};
///Result from a selfie check
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GelatoSelfieReport {
    ///ID of the [File](https://stripe.com/docs/api/files) holding the image of the identity document used in this check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<String>,
    ///Details on the verification error. Present when status is `unverified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<serde_json::Value>,
    ///ID of the [File](https://stripe.com/docs/api/files) holding the image of the selfie used in this check.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<String>,
    ///Status of this `selfie` check.
    pub status: String,
}
impl std::fmt::Display for GelatoSelfieReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}