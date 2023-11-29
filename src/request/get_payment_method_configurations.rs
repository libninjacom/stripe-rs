use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetPaymentMethodConfigurationsRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub application: Option<serde_json::Value>,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetPaymentMethodConfigurationsRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<
        PaymentMethodConfigResourcePaymentMethodConfigurationsList,
    > {
        let mut r = self.http_client.client.get("/v1/payment_method_configurations");
        if let Some(ref unwrapped) = self.application {
            r = r.query("application", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.query("expand[]", &item.to_string());
            }
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn application(mut self, application: serde_json::Value) -> Self {
        self.application = Some(application);
        self
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetPaymentMethodConfigurationsRequest<'a> {
    type Output = httpclient::InMemoryResult<
        PaymentMethodConfigResourcePaymentMethodConfigurationsList,
    >;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}