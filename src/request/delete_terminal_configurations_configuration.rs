use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteTerminalConfigurationsConfigurationRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub configuration: String,
}
impl<'a> DeleteTerminalConfigurationsConfigurationRequest<'a> {
    pub async fn send(
        self,
    ) -> ::httpclient::InMemoryResult<DeletedTerminalConfiguration> {
        let mut r = self
            .http_client
            .client
            .delete(
                &format!(
                    "/v1/terminal/configurations/{configuration}", configuration = self
                    .configuration
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture
for DeleteTerminalConfigurationsConfigurationRequest<'a> {
    type Output = httpclient::InMemoryResult<DeletedTerminalConfiguration>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}