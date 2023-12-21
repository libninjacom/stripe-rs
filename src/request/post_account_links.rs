use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_account_links`].

On request success, this will return a [`AccountLink`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostAccountLinksRequest {}
impl PostAccountLinksRequest {}
impl FluentRequest<'_, PostAccountLinksRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostAccountLinksRequest> {
    type Output = httpclient::InMemoryResult<AccountLink>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/account_links";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}