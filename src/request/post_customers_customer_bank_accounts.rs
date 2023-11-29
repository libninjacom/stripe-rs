use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCustomersCustomerBankAccountsRequest {
    pub customer: String,
}
impl PostCustomersCustomerBankAccountsRequest {}
impl FluentRequest<'_, PostCustomersCustomerBankAccountsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostCustomersCustomerBankAccountsRequest> {
    type Output = httpclient::InMemoryResult<PaymentSource>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = &format!(
                "/v1/customers/{customer}/bank_accounts", customer = self.params.customer
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}