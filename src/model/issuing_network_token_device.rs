use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingNetworkTokenDevice {
    ///An obfuscated ID derived from the device ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_fingerprint: Option<String>,
    ///The IP address of the device at provisioning time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
    ///The geographic latitude/longitude coordinates of the device at provisioning time. The format is [+-]decimal/[+-]decimal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<String>,
    ///The name of the device used for tokenization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///The phone number of the device used for tokenization.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,
    ///The type of device used for tokenization.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
}
impl std::fmt::Display for IssuingNetworkTokenDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}