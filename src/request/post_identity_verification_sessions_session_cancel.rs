use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_identity_verification_sessions_session_cancel`].

On request success, this will return a [`IdentityVerificationSession`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostIdentityVerificationSessionsSessionCancelRequest {
    pub session: String,
}
impl PostIdentityVerificationSessionsSessionCancelRequest {}
impl FluentRequest<'_, PostIdentityVerificationSessionsSessionCancelRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostIdentityVerificationSessionsSessionCancelRequest> {
    type Output = httpclient::InMemoryResult<IdentityVerificationSession>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/identity/verification_sessions/{session}/cancel", session = self
                .params.session
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}