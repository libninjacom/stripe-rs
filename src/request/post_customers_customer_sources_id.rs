use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_customers_customer_sources_id`].

On request success, this will return a [`serde_json::Value`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCustomersCustomerSourcesIdRequest {
    pub customer: String,
    pub id: String,
}
impl PostCustomersCustomerSourcesIdRequest {}
impl FluentRequest<'_, PostCustomersCustomerSourcesIdRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostCustomersCustomerSourcesIdRequest> {
    type Output = httpclient::InMemoryResult<serde_json::Value>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/customers/{customer}/sources/{id}", customer = self.params.customer,
                id = self.params.id
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}