use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_tax_transactions_create_reversal`].

On request success, this will return a [`TaxTransaction`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTaxTransactionsCreateReversalRequest {}
impl PostTaxTransactionsCreateReversalRequest {}
impl FluentRequest<'_, PostTaxTransactionsCreateReversalRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostTaxTransactionsCreateReversalRequest> {
    type Output = httpclient::InMemoryResult<TaxTransaction>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/tax/transactions/create_reversal";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}