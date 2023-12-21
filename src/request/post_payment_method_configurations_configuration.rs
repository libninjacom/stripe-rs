use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_payment_method_configurations_configuration`].

On request success, this will return a [`PaymentMethodConfiguration`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostPaymentMethodConfigurationsConfigurationRequest {
    pub configuration: String,
}
impl PostPaymentMethodConfigurationsConfigurationRequest {}
impl FluentRequest<'_, PostPaymentMethodConfigurationsConfigurationRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostPaymentMethodConfigurationsConfigurationRequest> {
    type Output = httpclient::InMemoryResult<PaymentMethodConfiguration>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/payment_method_configurations/{configuration}", configuration = self
                .params.configuration
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}