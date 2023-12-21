use serde::{Serialize, Deserialize};
use super::{AccountRequirementsAlternative, AccountRequirementsError};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountRequirements {
    ///Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternatives: Option<Vec<AccountRequirementsAlternative>>,
    ///Date by which the fields in `currently_due` must be collected to keep the account enabled. These fields may disable the account sooner if the next threshold is reached before they are collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_deadline: Option<i64>,
    ///Fields that need to be collected to keep the account enabled. If not collected by `current_deadline`, these fields appear in `past_due` as well, and the account is disabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currently_due: Option<Vec<String>>,
    ///If the account is disabled, this string describes why. [Learn more about handling verification issues](https://stripe.com/docs/connect/handling-api-verification). Can be `action_required.requested_capabilities`, `requirements.past_due`, `requirements.pending_verification`, `listed`, `platform_paused`, `rejected.fraud`, `rejected.incomplete_verification`, `rejected.listed`, `rejected.other`, `rejected.terms_of_service`, `under_review`, or `other`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    ///Fields that are `currently_due` and need to be collected again because validation or verification failed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<AccountRequirementsError>>,
    ///Fields that need to be collected assuming all volume thresholds are reached. As they become required, they appear in `currently_due` as well, and `current_deadline` becomes set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eventually_due: Option<Vec<String>>,
    ///Fields that weren't collected by `current_deadline`. These fields need to be collected to enable the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub past_due: Option<Vec<String>>,
    ///Fields that may become required depending on the results of verification or review. Will be an empty array unless an asynchronous verification is pending. If verification fails, these fields move to `eventually_due`, `currently_due`, or `past_due`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_verification: Option<Vec<String>>,
}
impl std::fmt::Display for AccountRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}