use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostTaxRegistrationsIdRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub id: String,
}
impl<'a> PostTaxRegistrationsIdRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<TaxRegistration> {
        let mut r = self
            .http_client
            .client
            .post(&format!("/v1/tax/registrations/{id}", id = self.id));
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostTaxRegistrationsIdRequest<'a> {
    type Output = httpclient::InMemoryResult<TaxRegistration>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}