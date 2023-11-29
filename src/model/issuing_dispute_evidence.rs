
use serde::{Serialize, Deserialize};
use super::{
    IssuingDisputeCanceledEvidence, IssuingDisputeDuplicateEvidence,
    IssuingDisputeFraudulentEvidence, IssuingDisputeMerchandiseNotAsDescribedEvidence,
    IssuingDisputeNotReceivedEvidence, IssuingDisputeOtherEvidence,
    IssuingDisputeServiceNotAsDescribedEvidence,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingDisputeEvidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<IssuingDisputeCanceledEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate: Option<IssuingDisputeDuplicateEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraudulent: Option<IssuingDisputeFraudulentEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchandise_not_as_described: Option<
        IssuingDisputeMerchandiseNotAsDescribedEvidence,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_received: Option<IssuingDisputeNotReceivedEvidence>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<IssuingDisputeOtherEvidence>,
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_not_as_described: Option<IssuingDisputeServiceNotAsDescribedEvidence>,
}
impl std::fmt::Display for IssuingDisputeEvidence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}