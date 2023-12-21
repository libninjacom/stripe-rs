use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_accounts`].

On request success, this will return a [`Account`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostAccountsRequest {}
impl PostAccountsRequest {}
impl FluentRequest<'_, PostAccountsRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostAccountsRequest> {
    type Output = httpclient::InMemoryResult<Account>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/accounts";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}