
use serde::{Serialize, Deserialize};
use super::{
    FundingInstructionsBankTransferAbaRecord, FundingInstructionsBankTransferIbanRecord,
    FundingInstructionsBankTransferSortCodeRecord,
    FundingInstructionsBankTransferSpeiRecord,
    FundingInstructionsBankTransferSwiftRecord,
    FundingInstructionsBankTransferZenginRecord,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FundingInstructionsBankTransferFinancialAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<FundingInstructionsBankTransferAbaRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<FundingInstructionsBankTransferIbanRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<FundingInstructionsBankTransferSortCodeRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spei: Option<FundingInstructionsBankTransferSpeiRecord>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_networks: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift: Option<FundingInstructionsBankTransferSwiftRecord>,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zengin: Option<FundingInstructionsBankTransferZenginRecord>,
}
impl std::fmt::Display for FundingInstructionsBankTransferFinancialAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}