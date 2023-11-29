use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteRadarValueListItemsItemRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub item: String,
}
impl<'a> DeleteRadarValueListItemsItemRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<DeletedRadarValueListItem> {
        let mut r = self
            .http_client
            .client
            .delete(&format!("/v1/radar/value_list_items/{item}", item = self.item));
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for DeleteRadarValueListItemsItemRequest<'a> {
    type Output = httpclient::InMemoryResult<DeletedRadarValueListItem>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}