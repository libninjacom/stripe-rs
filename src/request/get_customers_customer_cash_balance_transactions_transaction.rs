use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetCustomersCustomerCashBalanceTransactionsTransactionRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub customer: String,
    pub expand: Option<Vec<String>>,
    pub transaction: String,
}
impl<'a> GetCustomersCustomerCashBalanceTransactionsTransactionRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<CustomerCashBalanceTransaction> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v1/customers/{customer}/cash_balance_transactions/{transaction}",
                    customer = self.customer, transaction = self.transaction
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.query("expand[]", &item.to_string());
            }
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for GetCustomersCustomerCashBalanceTransactionsTransactionRequest<'a> {
    type Output = httpclient::InMemoryResult<CustomerCashBalanceTransaction>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}