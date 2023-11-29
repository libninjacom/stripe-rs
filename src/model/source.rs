
use serde::{Serialize, Deserialize};
use super::{
    SourceCodeVerificationFlow, SourceOrder, SourceReceiverFlow, SourceRedirectFlow,
    SourceTypeAchCreditTransfer, SourceTypeAchDebit, SourceTypeAcssDebit,
    SourceTypeAlipay, SourceTypeAuBecsDebit, SourceTypeBancontact, SourceTypeCard,
    SourceTypeCardPresent, SourceTypeEps, SourceTypeGiropay, SourceTypeIdeal,
    SourceTypeKlarna, SourceTypeMultibanco, SourceTypeP24, SourceTypeSepaDebit,
    SourceTypeSofort, SourceTypeThreeDSecure, SourceTypeWechat,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<SourceTypeAchCreditTransfer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_debit: Option<SourceTypeAchDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<SourceTypeAcssDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<SourceTypeAlipay>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<SourceTypeAuBecsDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<SourceTypeBancontact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<SourceTypeCard>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<SourceTypeCardPresent>,
    pub client_secret: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verification: Option<SourceCodeVerificationFlow>,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<SourceTypeEps>,
    pub flow: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<SourceTypeGiropay>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<SourceTypeIdeal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<SourceTypeKlarna>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<SourceTypeMultibanco>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<SourceTypeP24>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<SourceReceiverFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<SourceRedirectFlow>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<SourceTypeSepaDebit>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<SourceTypeSofort>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_order: Option<SourceOrder>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<SourceTypeThreeDSecure>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat: Option<SourceTypeWechat>,
}
impl std::fmt::Display for Source {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}