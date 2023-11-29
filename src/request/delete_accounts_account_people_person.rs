use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteAccountsAccountPeoplePersonRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub account: String,
    pub person: String,
}
impl<'a> DeleteAccountsAccountPeoplePersonRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<DeletedPerson> {
        let mut r = self
            .http_client
            .client
            .delete(
                &format!(
                    "/v1/accounts/{account}/people/{person}", account = self.account,
                    person = self.person
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for DeleteAccountsAccountPeoplePersonRequest<'a> {
    type Output = httpclient::InMemoryResult<DeletedPerson>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}