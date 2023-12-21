use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_payment_method_configurations`].

On request success, this will return a [`GetPaymentMethodConfigurationsResponse`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPaymentMethodConfigurationsRequest {
    pub application: Option<serde_json::Value>,
    pub expand: Option<Vec<String>>,
}
impl GetPaymentMethodConfigurationsRequest {}
impl FluentRequest<'_, GetPaymentMethodConfigurationsRequest> {
    pub fn application(mut self, application: serde_json::Value) -> Self {
        self.params.application = Some(application);
        self
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetPaymentMethodConfigurationsRequest> {
    type Output = httpclient::InMemoryResult<GetPaymentMethodConfigurationsResponse>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/payment_method_configurations";
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}