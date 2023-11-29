
use serde::{Serialize, Deserialize};
use super::{
    Source, SourceMandateNotificationAcssDebitData,
    SourceMandateNotificationBacsDebitData, SourceMandateNotificationSepaDebitData,
};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceMandateNotification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<SourceMandateNotificationAcssDebitData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<SourceMandateNotificationBacsDebitData>,
    pub created: i64,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    pub reason: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<SourceMandateNotificationSepaDebitData>,
    pub source: Source,
    pub status: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for SourceMandateNotification {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}