use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteTerminalLocationsLocationRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub location: String,
}
impl<'a> DeleteTerminalLocationsLocationRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<DeletedTerminalLocation> {
        let mut r = self
            .http_client
            .client
            .delete(
                &format!("/v1/terminal/locations/{location}", location = self.location),
            );
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for DeleteTerminalLocationsLocationRequest<'a> {
    type Output = httpclient::InMemoryResult<DeletedTerminalLocation>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}