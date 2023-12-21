use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TerminalConfigurationConfigurationResourceCurrencySpecificConfig {
    ///Fixed amounts displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,
    ///Percentages displayed when collecting a tip
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,
    ///Below this amount, fixed amounts will be displayed; above it, percentages will be displayed
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}
impl std::fmt::Display
for TerminalConfigurationConfigurationResourceCurrencySpecificConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}