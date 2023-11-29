use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetChargesSearchRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub limit: Option<i64>,
    pub page: Option<String>,
    pub query: String,
}
impl<'a> GetChargesSearchRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<SearchResult> {
        let mut r = self.http_client.client.get("/v1/charges/search");
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.query("expand[]", &item.to_string());
            }
        }
        if let Some(ref unwrapped) = self.limit {
            r = r.query("limit", &unwrapped.to_string());
        }
        if let Some(ref unwrapped) = self.page {
            r = r.query("page", &unwrapped.to_string());
        }
        r = r.query("query", &self.query.to_string());
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
    pub fn page(mut self, page: &str) -> Self {
        self.page = Some(page.to_owned());
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetChargesSearchRequest<'a> {
    type Output = httpclient::InMemoryResult<SearchResult>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}