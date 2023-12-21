use serde::{Serialize, Deserialize};
use super::FundingInstructionsBankTransfer;
/**Each customer has a [`balance`](https://stripe.com/docs/api/customers/object#customer_object-balance) that is
automatically applied to future invoices and payments using the `customer_balance` payment method.
Customers can fund this balance by initiating a bank transfer to any account in the
`financial_addresses` field.
Related guide: [Customer balance funding instructions](https://stripe.com/docs/payments/customer-balance/funding-instructions)*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FundingInstructions {
    ///
    pub bank_transfer: FundingInstructionsBankTransfer,
    ///Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase. Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: String,
    ///The `funding_type` of the returned instructions
    pub funding_type: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
}
impl std::fmt::Display for FundingInstructions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}