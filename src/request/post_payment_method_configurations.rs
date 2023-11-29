use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostPaymentMethodConfigurationsRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
}
impl<'a> PostPaymentMethodConfigurationsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<PaymentMethodConfiguration> {
        let mut r = self.http_client.client.post("/v1/payment_method_configurations");
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostPaymentMethodConfigurationsRequest<'a> {
    type Output = httpclient::InMemoryResult<PaymentMethodConfiguration>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}