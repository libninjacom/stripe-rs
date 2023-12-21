use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SetupAttemptPaymentMethodDetailsCard {
    ///Card brand. Can be `amex`, `diners`, `discover`, `eftpos_au`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub brand: Option<String>,
    ///Check results by Card networks on Card address and CVC at the time of authorization
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checks: Option<serde_json::Value>,
    ///Two-letter ISO code representing the country of the card. You could use this attribute to get a sense of the international breakdown of cards you've collected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,
    ///Two-digit number representing the card's expiration month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,
    ///Four-digit number representing the card's expiration year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,
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
    ///Identifies which network this charge was processed on. Can be `amex`, `cartes_bancaires`, `diners`, `discover`, `eftpos_au`, `interac`, `jcb`, `mastercard`, `unionpay`, `visa`, or `unknown`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<String>,
    ///Populated if this authorization used 3D Secure authentication.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<serde_json::Value>,
    ///If this Card is part of a card wallet, this contains the details of the card wallet.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<serde_json::Value>,
}
impl std::fmt::Display for SetupAttemptPaymentMethodDetailsCard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}