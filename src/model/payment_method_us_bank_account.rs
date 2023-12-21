use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodUsBankAccount {
    ///Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<String>,
    ///Account type: checkings or savings. Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    ///The name of the bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,
    ///The ID of the Financial Connections Account used to create the payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<String>,
    ///Uniquely identifies this particular bank account. You can use this attribute to check whether two bank accounts are the same.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    ///Last four digits of the bank account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    ///Contains information about US bank account networks that can be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<serde_json::Value>,
    ///Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
    ///Contains information about the future reusability of this PaymentMethod.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<serde_json::Value>,
}
impl std::fmt::Display for PaymentMethodUsBankAccount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}