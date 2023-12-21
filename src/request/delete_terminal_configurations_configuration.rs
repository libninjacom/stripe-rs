use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::delete_terminal_configurations_configuration`].

On request success, this will return a [`DeletedTerminalConfiguration`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteTerminalConfigurationsConfigurationRequest {
    pub configuration: String,
}
impl DeleteTerminalConfigurationsConfigurationRequest {}
impl FluentRequest<'_, DeleteTerminalConfigurationsConfigurationRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, DeleteTerminalConfigurationsConfigurationRequest> {
    type Output = httpclient::InMemoryResult<DeletedTerminalConfiguration>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/terminal/configurations/{configuration}", configuration = self
                .params.configuration
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}