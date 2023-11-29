use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetEventsIdRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub id: String,
}
impl<'a> GetEventsIdRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Event> {
        let mut r = self
            .http_client
            .client
            .get(&format!("/v1/events/{id}", id = self.id));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.query("expand[]", &item.to_string());
            }
        }
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetEventsIdRequest<'a> {
    type Output = httpclient::InMemoryResult<Event>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}