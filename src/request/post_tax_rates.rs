use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_tax_rates`].

On request success, this will return a [`TaxRate`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTaxRatesRequest {}
impl PostTaxRatesRequest {}
impl FluentRequest<'_, PostTaxRatesRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostTaxRatesRequest> {
    type Output = httpclient::InMemoryResult<TaxRate>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/tax_rates";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}