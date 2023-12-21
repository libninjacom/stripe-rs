use serde::{Serialize, Deserialize};
///Zengin Records contain Japan bank account details per the Zengin format.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FundingInstructionsBankTransferZenginRecord {
    ///The account holder name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,
    ///The account number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,
    ///The bank account type. In Japan, this can only be `futsu` or `toza`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    ///The bank code of the account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,
    ///The bank name of the account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    ///The branch code of the account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_code: Option<String>,
    ///The branch name of the account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_name: Option<String>,
}
impl std::fmt::Display for FundingInstructionsBankTransferZenginRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}