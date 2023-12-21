use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_identity_verification_sessions_session_redact`].

On request success, this will return a [`IdentityVerificationSession`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostIdentityVerificationSessionsSessionRedactRequest {
    pub session: String,
}
impl PostIdentityVerificationSessionsSessionRedactRequest {}
impl FluentRequest<'_, PostIdentityVerificationSessionsSessionRedactRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostIdentityVerificationSessionsSessionRedactRequest> {
    type Output = httpclient::InMemoryResult<IdentityVerificationSession>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/identity/verification_sessions/{session}/redact", session = self
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