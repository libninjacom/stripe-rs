use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DisputeEvidenceDetails {
    ///Date by which evidence must be submitted in order to successfully challenge dispute. Will be 0 if the customer's bank or credit card company doesn't allow a response for this particular dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_by: Option<i64>,
    ///Whether evidence has been staged for this dispute.
    pub has_evidence: bool,
    ///Whether the last evidence submission was submitted past the due date. Defaults to `false` if no evidence submissions have occurred. If `true`, then delivery of the latest evidence is *not* guaranteed.
    pub past_due: bool,
    ///The number of times evidence has been submitted. Typically, you may only submit evidence once.
    pub submission_count: i64,
}
impl std::fmt::Display for DisputeEvidenceDetails {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}