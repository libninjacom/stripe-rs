use serde::{Serialize, Deserialize};
use super::{
    GelatoDocumentReport, GelatoIdNumberReport, GelatoSelfieReport,
    GelatoVerificationReportOptions,
};
/**A VerificationReport is the result of an attempt to collect and verify data from a user.
The collection of verification checks performed is determined from the `type` and `options`
parameters used. You can find the result of each verification check performed in the
appropriate sub-resource: `document`, `id_number`, `selfie`.

Each VerificationReport contains a copy of any data collected by the user as well as
reference IDs which can be used to access collected images through the [FileUpload](https://stripe.com/docs/api/files)
API. To configure and create VerificationReports, use the
[VerificationSession](https://stripe.com/docs/api/identity/verification_sessions) API.

Related guides: [Accessing verification results](https://stripe.com/docs/identity/verification-sessions#results).*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IdentityVerificationReport {
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Result from a document check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<GelatoDocumentReport>,
    ///Unique identifier for the object.
    pub id: String,
    ///Result from an id_number check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<GelatoIdNumberReport>,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<GelatoVerificationReportOptions>,
    ///Result from a selfie check
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<GelatoSelfieReport>,
    ///Type of report.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    ///ID of the VerificationSession that created this report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_session: Option<String>,
}
impl std::fmt::Display for IdentityVerificationReport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}