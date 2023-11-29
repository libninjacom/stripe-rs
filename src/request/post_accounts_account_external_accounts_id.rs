use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostAccountsAccountExternalAccountsIdRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub account: String,
    pub id: String,
}
impl<'a> PostAccountsAccountExternalAccountsIdRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<ExternalAccount> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v1/accounts/{account}/external_accounts/{id}", account = self
                    .account, id = self.id
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostAccountsAccountExternalAccountsIdRequest<'a> {
    type Output = httpclient::InMemoryResult<ExternalAccount>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}