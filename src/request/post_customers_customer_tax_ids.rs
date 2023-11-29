use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostCustomersCustomerTaxIdsRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub customer: String,
}
impl<'a> PostCustomersCustomerTaxIdsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<TaxId> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!("/v1/customers/{customer}/tax_ids", customer = self.customer),
            );
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostCustomersCustomerTaxIdsRequest<'a> {
    type Output = httpclient::InMemoryResult<TaxId>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}