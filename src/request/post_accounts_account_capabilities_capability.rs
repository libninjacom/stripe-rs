use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostAccountsAccountCapabilitiesCapabilityRequest {
    pub account: String,
    pub capability: String,
}
impl PostAccountsAccountCapabilitiesCapabilityRequest {}
impl FluentRequest<'_, PostAccountsAccountCapabilitiesCapabilityRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostAccountsAccountCapabilitiesCapabilityRequest> {
    type Output = httpclient::InMemoryResult<Capability>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = &format!(
                "/v1/accounts/{account}/capabilities/{capability}", account = self.params
                .account, capability = self.params.capability
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}