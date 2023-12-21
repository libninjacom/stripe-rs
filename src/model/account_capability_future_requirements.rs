use serde::{Serialize, Deserialize};
use super::{AccountRequirementsAlternative, AccountRequirementsError};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AccountCapabilityFutureRequirements {
    ///Fields that are due and can be satisfied by providing the corresponding alternative fields instead.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternatives: Option<Vec<AccountRequirementsAlternative>>,
    ///Date on which `future_requirements` merges with the main `requirements` hash and `future_requirements` becomes empty. After the transition, `currently_due` requirements may immediately become `past_due`, but the account may also be given a grace period depending on the capability's enablement state prior to transitioning.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_deadline: Option<i64>,
    ///Fields that need to be collected to keep the capability enabled. If not collected by `future_requirements[current_deadline]`, these fields will transition to the main `requirements` hash.
    pub currently_due: Vec<String>,
    ///This is typed as a string for consistency with `requirements.disabled_reason`, but it safe to assume `future_requirements.disabled_reason` is empty because fields in `future_requirements` will never disable the account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_reason: Option<String>,
    ///Fields that are `currently_due` and need to be collected again because validation or verification failed.
    pub errors: Vec<AccountRequirementsError>,
    ///Fields that need to be collected assuming all volume thresholds are reached. As they become required, they appear in `currently_due` as well.
    pub eventually_due: Vec<String>,
    ///Fields that weren't collected by `requirements.current_deadline`. These fields need to be collected to enable the capability on the account. New fields will never appear here; `future_requirements.past_due` will always be a subset of `requirements.past_due`.
    pub past_due: Vec<String>,
    ///Fields that may become required depending on the results of verification or review. Will be an empty array unless an asynchronous verification is pending. If verification fails, these fields move to `eventually_due` or `currently_due`.
    pub pending_verification: Vec<String>,
}
impl std::fmt::Display for AccountCapabilityFutureRequirements {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}