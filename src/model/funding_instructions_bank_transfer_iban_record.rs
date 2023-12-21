use serde::{Serialize, Deserialize};
///Iban Records contain E.U. bank account details per the SEPA format.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FundingInstructionsBankTransferIbanRecord {
    ///The name of the person or business that owns the bank account
    pub account_holder_name: String,
    ///The BIC/SWIFT code of the account.
    pub bic: String,
    ///Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,
    ///The IBAN of the account.
    pub iban: String,
}
impl std::fmt::Display for FundingInstructionsBankTransferIbanRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}