use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostSetupIntentsIntentVerifyMicrodepositsRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub intent: String,
}
impl<'a> PostSetupIntentsIntentVerifyMicrodepositsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<SetupIntent> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v1/setup_intents/{intent}/verify_microdeposits", intent = self
                    .intent
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture
for PostSetupIntentsIntentVerifyMicrodepositsRequest<'a> {
    type Output = httpclient::InMemoryResult<SetupIntent>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}