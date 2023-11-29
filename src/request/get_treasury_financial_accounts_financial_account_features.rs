use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTreasuryFinancialAccountsFinancialAccountFeaturesRequest {
    pub expand: Option<Vec<String>>,
    pub financial_account: String,
}
impl GetTreasuryFinancialAccountsFinancialAccountFeaturesRequest {}
impl FluentRequest<'_, GetTreasuryFinancialAccountsFinancialAccountFeaturesRequest> {
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetTreasuryFinancialAccountsFinancialAccountFeaturesRequest> {
    type Output = httpclient::InMemoryResult<TreasuryFinancialAccountFeatures>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async {
            let url = &format!(
                "/v1/treasury/financial_accounts/{financial_account}/features",
                financial_account = self.params.financial_account
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}