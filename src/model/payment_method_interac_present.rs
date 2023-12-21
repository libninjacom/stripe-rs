use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodInteracPresent {
    ///Card brand. Can be `interac`, `mastercard` or `visa`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    ///The cardholder name as read from the card, in [ISO 7813](https://en.wikipedia.org/wiki/ISO/IEC_7813) format. May include alphanumeric characters, special characters and first/last name separator (`/`). In some cases, the cardholder name may not be available depending on how the issuer has configured the card. Cardholder name is typically not available on swipe or contactless payments, such as those made with Apple Pay and Google Pay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder_name: Option<String>,
    ///Two-letter ISO code representing the country of the card. You could use this attribute to get a sense of the international breakdown of cards you've collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    ///Two-digit number representing the card's expiration month.
    pub exp_month: i64,
    ///Four-digit number representing the card's expiration year.
    pub exp_year: i64,
    /**Uniquely identifies this particular card number. You can use this attribute to check whether two customers who’ve signed up with you are using the same card number, for example. For payment methods that tokenize card information (Apple Pay, Google Pay), the tokenized number might be provided instead of the underlying card number.

*As of May 1, 2021, card fingerprint in India for Connect changed to allow two fingerprints for the same card---one for India and one for the rest of the world.**/
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,
    ///Card funding type. Can be `credit`, `debit`, `prepaid`, or `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding: Option<String>,
    ///The last four digits of the card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
    ///Contains information about card networks that can be used to process the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub networks: Option<serde_json::Value>,
    ///EMV tag 5F2D. Preferred languages specified by the integrated circuit chip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,
    ///How card details were read in this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_method: Option<String>,
}
impl std::fmt::Display for PaymentMethodInteracPresent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}