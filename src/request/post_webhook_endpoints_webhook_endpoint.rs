use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostWebhookEndpointsWebhookEndpointRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub webhook_endpoint: String,
}
impl<'a> PostWebhookEndpointsWebhookEndpointRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<WebhookEndpoint> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v1/webhook_endpoints/{webhook_endpoint}", webhook_endpoint = self
                    .webhook_endpoint
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostWebhookEndpointsWebhookEndpointRequest<'a> {
    type Output = httpclient::InMemoryResult<WebhookEndpoint>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}