use serde::{Serialize, Deserialize};
///
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PaymentMethodBoleto {
    ///Uniquely identifies the customer tax id (CNPJ or CPF)
    pub tax_id: String,
}
impl std::fmt::Display for PaymentMethodBoleto {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}