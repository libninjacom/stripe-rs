use serde::{Serialize, Deserialize};
use super::{
    IssuingNetworkTokenDevice, IssuingNetworkTokenMastercard, IssuingNetworkTokenVisa,
    IssuingNetworkTokenWalletProvider,
};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct IssuingNetworkTokenNetworkData {
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<IssuingNetworkTokenDevice>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mastercard: Option<IssuingNetworkTokenMastercard>,
    ///The network that the token is associated with. An additional hash is included with a name matching this value, containing tokenization data specific to the card network.
    #[serde(rename = "type")]
    pub type_: String,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa: Option<IssuingNetworkTokenVisa>,
    ///
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet_provider: Option<IssuingNetworkTokenWalletProvider>,
}
impl std::fmt::Display for IssuingNetworkTokenNetworkData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}