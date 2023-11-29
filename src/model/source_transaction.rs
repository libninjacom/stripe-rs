
use serde::{Serialize, Deserialize};
use super::{
    SourceTransactionAchCreditTransferData, SourceTransactionChfCreditTransferData,
    SourceTransactionGbpCreditTransferData, SourceTransactionPaperCheckData,
    SourceTransactionSepaCreditTransferData,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SourceTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<SourceTransactionAchCreditTransferData>,
    pub amount: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf_credit_transfer: Option<SourceTransactionChfCreditTransferData>,
    pub created: i64,
    pub currency: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp_credit_transfer: Option<SourceTransactionGbpCreditTransferData>,
    pub id: String,
    pub livemode: bool,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_check: Option<SourceTransactionPaperCheckData>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_credit_transfer: Option<SourceTransactionSepaCreditTransferData>,
    pub source: String,
    pub status: String,
    #[serde(rename = "type")]
    pub type_: String,
}
impl std::fmt::Display for SourceTransaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}