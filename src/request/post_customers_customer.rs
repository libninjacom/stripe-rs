use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_customers_customer`].

On request success, this will return a [`Customer`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCustomersCustomerRequest {
    pub customer: String,
}
impl PostCustomersCustomerRequest {}
impl FluentRequest<'_, PostCustomersCustomerRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostCustomersCustomerRequest> {
    type Output = httpclient::InMemoryResult<Customer>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/customers/{customer}", customer = self.params.customer
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}