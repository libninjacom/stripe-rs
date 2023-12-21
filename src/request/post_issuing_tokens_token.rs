use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_issuing_tokens_token`].

On request success, this will return a [`IssuingToken`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostIssuingTokensTokenRequest {
    pub token: String,
}
impl PostIssuingTokensTokenRequest {}
impl FluentRequest<'_, PostIssuingTokensTokenRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostIssuingTokensTokenRequest> {
    type Output = httpclient::InMemoryResult<IssuingToken>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/issuing/tokens/{token}", token = self.params.token);
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}