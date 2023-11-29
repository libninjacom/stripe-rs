use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostTaxSettingsRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
}
impl<'a> PostTaxSettingsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<TaxSettings> {
        let mut r = self.http_client.client.post("/v1/tax/settings");
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostTaxSettingsRequest<'a> {
    type Output = httpclient::InMemoryResult<TaxSettings>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}