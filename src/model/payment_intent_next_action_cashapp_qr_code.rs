
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentIntentNextActionCashappQrCode {
    pub expires_at: i64,
    pub image_url_png: String,
    pub image_url_svg: String,
}
impl std::fmt::Display for PaymentIntentNextActionCashappQrCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}