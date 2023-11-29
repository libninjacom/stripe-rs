use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteWebhookEndpointsWebhookEndpointRequest {
    pub webhook_endpoint: String,
}
impl DeleteWebhookEndpointsWebhookEndpointRequest {}
impl FluentRequest<'_, DeleteWebhookEndpointsWebhookEndpointRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, DeleteWebhookEndpointsWebhookEndpointRequest> {
    type Output = httpclient::InMemoryResult<DeletedWebhookEndpoint>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = &format!(
                "/v1/webhook_endpoints/{webhook_endpoint}", webhook_endpoint = self
                .params.webhook_endpoint
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}