use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_treasury_credit_reversals_credit_reversal`].

On request success, this will return a [`TreasuryCreditReversal`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetTreasuryCreditReversalsCreditReversalRequest {
    pub credit_reversal: String,
    pub expand: Option<Vec<String>>,
}
impl GetTreasuryCreditReversalsCreditReversalRequest {}
impl FluentRequest<'_, GetTreasuryCreditReversalsCreditReversalRequest> {
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetTreasuryCreditReversalsCreditReversalRequest> {
    type Output = httpclient::InMemoryResult<TreasuryCreditReversal>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/treasury/credit_reversals/{credit_reversal}", credit_reversal = self
                .params.credit_reversal
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}