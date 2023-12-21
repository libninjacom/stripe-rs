use serde::{Serialize, Deserialize};
use super::{
    FundingInstructionsBankTransferAbaRecord, FundingInstructionsBankTransferIbanRecord,
    FundingInstructionsBankTransferSortCodeRecord,
    FundingInstructionsBankTransferSpeiRecord,
    FundingInstructionsBankTransferSwiftRecord,
    FundingInstructionsBankTransferZenginRecord,
};
///FinancialAddresses contain identifying information that resolves to a FinancialAccount.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FundingInstructionsBankTransferFinancialAddress {
    ///ABA Records contain U.S. bank account details per the ABA format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<FundingInstructionsBankTransferAbaRecord>,
    ///Iban Records contain E.U. bank account details per the SEPA format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<FundingInstructionsBankTransferIbanRecord>,
    ///Sort Code Records contain U.K. bank account details per the sort code format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<FundingInstructionsBankTransferSortCodeRecord>,
    ///SPEI Records contain Mexico bank account details per the SPEI format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spei: Option<FundingInstructionsBankTransferSpeiRecord>,
    ///The payment networks supported by this FinancialAddress
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_networks: Option<Vec<String>>,
    ///SWIFT Records contain U.S. bank account details per the SWIFT format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub swift: Option<FundingInstructionsBankTransferSwiftRecord>,
    ///The type of financial address
    #[serde(rename = "type")]
    pub type_: String,
    ///Zengin Records contain Japan bank account details per the Zengin format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub zengin: Option<FundingInstructionsBankTransferZenginRecord>,
}
impl std::fmt::Display for FundingInstructionsBankTransferFinancialAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}