
use serde::{Serialize, Deserialize};
use super::{
    ApmsSourcesSourceList, CustomerTax, InvoiceSettingCustomerSetting, SubscriptionList,
    TaxIDsList,
};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Customer {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_balance: Option<serde_json::Value>,
    pub created: i64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delinquent: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_credit_balance: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<InvoiceSettingCustomerSetting>,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invoice_sequence: Option<i64>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sources: Option<ApmsSourcesSourceList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscriptions: Option<SubscriptionList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<CustomerTax>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<TaxIDsList>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<serde_json::Value>,
}
impl std::fmt::Display for Customer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}