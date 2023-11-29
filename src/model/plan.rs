
use serde::{Serialize, Deserialize};
use super::PlanTier;
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct Plan {
    pub active: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aggregate_usage: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(with = "rust_decimal::serde::str_option")]
    pub amount_decimal: Option<rust_decimal::Decimal>,
    pub billing_scheme: String,
    pub created: i64,
    pub currency: String,
    pub id: String,
    pub interval: String,
    pub interval_count: i64,
    pub livemode: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    pub object: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers: Option<Vec<PlanTier>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tiers_mode: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transform_usage: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<i64>,
    pub usage_type: String,
}
impl std::fmt::Display for Plan {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}