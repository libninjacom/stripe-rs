use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetSourcesSourceSourceTransactionsSourceTransactionRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub source: String,
    pub source_transaction: String,
}
impl<'a> GetSourcesSourceSourceTransactionsSourceTransactionRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<SourceTransaction> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v1/sources/{source}/source_transactions/{source_transaction}",
                    source = self.source, source_transaction = self.source_transaction
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.query("expand[]", &item.to_string());
            }
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for GetSourcesSourceSourceTransactionsSourceTransactionRequest<'a> {
    type Output = httpclient::InMemoryResult<SourceTransaction>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}