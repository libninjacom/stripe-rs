use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostIdentityVerificationSessionsSessionCancelRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub session: String,
}
impl<'a> PostIdentityVerificationSessionsSessionCancelRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<IdentityVerificationSession> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v1/identity/verification_sessions/{session}/cancel", session = self
                    .session
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture
for PostIdentityVerificationSessionsSessionCancelRequest<'a> {
    type Output = httpclient::InMemoryResult<IdentityVerificationSession>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}