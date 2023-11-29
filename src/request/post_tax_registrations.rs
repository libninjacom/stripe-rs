use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostTaxRegistrationsRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
}
impl<'a> PostTaxRegistrationsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<TaxRegistration> {
        let mut r = self.http_client.client.post("/v1/tax/registrations");
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostTaxRegistrationsRequest<'a> {
    type Output = httpclient::InMemoryResult<TaxRegistration>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}