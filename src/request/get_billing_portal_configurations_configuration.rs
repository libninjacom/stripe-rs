use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_billing_portal_configurations_configuration`].

On request success, this will return a [`BillingPortalConfiguration`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetBillingPortalConfigurationsConfigurationRequest {
    pub configuration: String,
    pub expand: Option<Vec<String>>,
}
impl GetBillingPortalConfigurationsConfigurationRequest {}
impl FluentRequest<'_, GetBillingPortalConfigurationsConfigurationRequest> {
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetBillingPortalConfigurationsConfigurationRequest> {
    type Output = httpclient::InMemoryResult<BillingPortalConfiguration>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/billing_portal/configurations/{configuration}", configuration = self
                .params.configuration
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}