use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_identity_verification_sessions`].

On request success, this will return a [`IdentityVerificationSession`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostIdentityVerificationSessionsRequest {}
impl PostIdentityVerificationSessionsRequest {}
impl FluentRequest<'_, PostIdentityVerificationSessionsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostIdentityVerificationSessionsRequest> {
    type Output = httpclient::InMemoryResult<IdentityVerificationSession>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/identity/verification_sessions";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}