use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_ephemeral_keys`].

On request success, this will return a [`EphemeralKey`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostEphemeralKeysRequest {}
impl PostEphemeralKeysRequest {}
impl FluentRequest<'_, PostEphemeralKeysRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostEphemeralKeysRequest> {
    type Output = httpclient::InMemoryResult<EphemeralKey>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/ephemeral_keys";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}