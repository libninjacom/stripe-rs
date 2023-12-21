use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_account_sessions`].

On request success, this will return a [`AccountSession`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostAccountSessionsRequest {}
impl PostAccountSessionsRequest {}
impl FluentRequest<'_, PostAccountSessionsRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostAccountSessionsRequest> {
    type Output = httpclient::InMemoryResult<AccountSession>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/account_sessions";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}