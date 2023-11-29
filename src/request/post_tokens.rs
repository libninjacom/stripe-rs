use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostTokensRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
}
impl<'a> PostTokensRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Token> {
        let mut r = self.http_client.client.post("/v1/tokens");
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostTokensRequest<'a> {
    type Output = httpclient::InMemoryResult<Token>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}