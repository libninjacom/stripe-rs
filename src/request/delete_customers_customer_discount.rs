use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteCustomersCustomerDiscountRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub customer: String,
}
impl<'a> DeleteCustomersCustomerDiscountRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<DeletedDiscount> {
        let mut r = self
            .http_client
            .client
            .delete(
                &format!("/v1/customers/{customer}/discount", customer = self.customer),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for DeleteCustomersCustomerDiscountRequest<'a> {
    type Output = httpclient::InMemoryResult<DeletedDiscount>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}