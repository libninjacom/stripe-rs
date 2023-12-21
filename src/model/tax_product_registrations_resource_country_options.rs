use serde::{Serialize, Deserialize};
use super::{
    TaxProductRegistrationsResourceCountryOptionsCanada,
    TaxProductRegistrationsResourceCountryOptionsDefault,
    TaxProductRegistrationsResourceCountryOptionsEurope,
    TaxProductRegistrationsResourceCountryOptionsSimplified,
    TaxProductRegistrationsResourceCountryOptionsUnitedStates,
};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TaxProductRegistrationsResourceCountryOptions {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ae: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub at: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub be: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bg: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ca: Option<TaxProductRegistrationsResourceCountryOptionsCanada>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ch: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cl: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub co: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cy: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cz: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub de: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dk: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ee: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub es: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fi: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fr: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gb: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gr: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hr: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hu: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ie: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub it: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jp: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kr: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lu: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lv: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mt: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mx: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub my: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nl: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nz: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pl: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pt: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ro: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sa: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub se: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sg: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub si: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sk: Option<TaxProductRegistrationsResourceCountryOptionsEurope>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub th: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tr: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us: Option<TaxProductRegistrationsResourceCountryOptionsUnitedStates>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vn: Option<TaxProductRegistrationsResourceCountryOptionsSimplified>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub za: Option<TaxProductRegistrationsResourceCountryOptionsDefault>,
}
impl std::fmt::Display for TaxProductRegistrationsResourceCountryOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}