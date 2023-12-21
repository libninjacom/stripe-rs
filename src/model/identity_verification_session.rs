use serde::{Serialize, Deserialize};
/**A VerificationSession guides you through the process of collecting and verifying the identities
of your users. It contains details about the type of verification, such as what [verification
check](/docs/identity/verification-checks) to perform. Only create one VerificationSession for
each verification in your system.

A VerificationSession transitions through [multiple
statuses](/docs/identity/how-sessions-work) throughout its lifetime as it progresses through
the verification flow. The VerificationSession contains the user's verified data after
verification checks are complete.

Related guide: [The Verification Sessions API](https://stripe.com/docs/identity/verification-sessions)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationSession {
    ///The short-lived client secret used by Stripe.js to [show a verification modal](https://stripe.com/docs/js/identity/modal) inside your app. This client secret expires after 24 hours and can only be used once. Don’t store it, log it, embed it in a URL, or expose it to anyone other than the user. Make sure that you have TLS enabled on any page that includes the client secret. Refer to our docs on [passing the client secret to the frontend](https://stripe.com/docs/identity/verification-sessions#client-secret) to learn more.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Unique identifier for the object.
    pub id: String,
    ///If present, this property tells you the last error encountered when processing the verification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<serde_json::Value>,
    ///ID of the most recent VerificationReport. [Learn more about accessing detailed verification results.](https://stripe.com/docs/identity/verification-sessions#results)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_verification_report: Option<serde_json::Value>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object. This can be useful for storing additional information about the object in a structured format.
    pub metadata: serde_json::Value,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///A set of options for the session’s verification checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<serde_json::Value>,
    ///Redaction status of this VerificationSession. If the VerificationSession is not redacted, this field will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redaction: Option<serde_json::Value>,
    ///Status of this VerificationSession. [Learn more about the lifecycle of sessions](https://stripe.com/docs/identity/how-sessions-work).
    pub status: String,
    ///The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///The short-lived URL that you use to redirect a user to Stripe to submit their identity information. This URL expires after 48 hours and can only be used once. Don’t store it, log it, send it in emails or expose it to anyone other than the user. Refer to our docs on [verifying identity documents](https://stripe.com/docs/identity/verify-identity-documents?platform=web&type=redirect) to learn how to redirect users to Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    ///The user’s verified data.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verified_outputs: Option<serde_json::Value>,
}
impl std::fmt::Display for IdentityVerificationSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}