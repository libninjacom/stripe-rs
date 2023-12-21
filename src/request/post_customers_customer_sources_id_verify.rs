use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_customers_customer_sources_id_verify`].

On request success, this will return a [`BankAccount`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCustomersCustomerSourcesIdVerifyRequest {
    pub customer: String,
    pub id: String,
}
impl PostCustomersCustomerSourcesIdVerifyRequest {}
impl FluentRequest<'_, PostCustomersCustomerSourcesIdVerifyRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostCustomersCustomerSourcesIdVerifyRequest> {
    type Output = httpclient::InMemoryResult<BankAccount>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/customers/{customer}/sources/{id}/verify", customer = self.params
                .customer, id = self.params.id
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}