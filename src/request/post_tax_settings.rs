use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_tax_settings`].

On request success, this will return a [`TaxSettings`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTaxSettingsRequest {}
impl PostTaxSettingsRequest {}
impl FluentRequest<'_, PostTaxSettingsRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostTaxSettingsRequest> {
    type Output = httpclient::InMemoryResult<TaxSettings>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/tax/settings";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}