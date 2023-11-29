use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTaxRatesTaxRateRequest {
    pub tax_rate: String,
}
impl PostTaxRatesTaxRateRequest {}
impl FluentRequest<'_, PostTaxRatesTaxRateRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostTaxRatesTaxRateRequest> {
    type Output = httpclient::InMemoryResult<TaxRate>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = &format!(
                "/v1/tax_rates/{tax_rate}", tax_rate = self.params.tax_rate
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}