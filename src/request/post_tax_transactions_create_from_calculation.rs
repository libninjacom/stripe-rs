use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_tax_transactions_create_from_calculation`].

On request success, this will return a [`TaxTransaction`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTaxTransactionsCreateFromCalculationRequest {}
impl PostTaxTransactionsCreateFromCalculationRequest {}
impl FluentRequest<'_, PostTaxTransactionsCreateFromCalculationRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostTaxTransactionsCreateFromCalculationRequest> {
    type Output = httpclient::InMemoryResult<TaxTransaction>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/tax/transactions/create_from_calculation";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}