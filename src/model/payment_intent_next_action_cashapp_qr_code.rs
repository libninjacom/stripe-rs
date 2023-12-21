use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentIntentNextActionCashappQrCode {
    ///The date (unix timestamp) when the QR code expires.
    pub expires_at: i64,
    ///The image_url_png string used to render QR code
    pub image_url_png: String,
    ///The image_url_svg string used to render QR code
    pub image_url_svg: String,
}
impl std::fmt::Display for PaymentIntentNextActionCashappQrCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}