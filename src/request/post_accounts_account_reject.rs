use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostAccountsAccountRejectRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub account: String,
}
impl<'a> PostAccountsAccountRejectRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Account> {
        let mut r = self
            .http_client
            .client
            .post(&format!("/v1/accounts/{account}/reject", account = self.account));
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostAccountsAccountRejectRequest<'a> {
    type Output = httpclient::InMemoryResult<Account>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}