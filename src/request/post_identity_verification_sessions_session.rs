use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_identity_verification_sessions_session`].

On request success, this will return a [`IdentityVerificationSession`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostIdentityVerificationSessionsSessionRequest {
    pub session: String,
}
impl PostIdentityVerificationSessionsSessionRequest {}
impl FluentRequest<'_, PostIdentityVerificationSessionsSessionRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostIdentityVerificationSessionsSessionRequest> {
    type Output = httpclient::InMemoryResult<IdentityVerificationSession>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/identity/verification_sessions/{session}", session = self.params
                .session
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}