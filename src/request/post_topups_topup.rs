use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_topups_topup`].

On request success, this will return a [`Topup`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTopupsTopupRequest {
    pub topup: String,
}
impl PostTopupsTopupRequest {}
impl FluentRequest<'_, PostTopupsTopupRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostTopupsTopupRequest> {
    type Output = httpclient::InMemoryResult<Topup>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/topups/{topup}", topup = self.params.topup);
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}