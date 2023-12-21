use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::delete_radar_value_list_items_item`].

On request success, this will return a [`DeletedRadarValueListItem`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRadarValueListItemsItemRequest {
    pub item: String,
}
impl DeleteRadarValueListItemsItemRequest {}
impl FluentRequest<'_, DeleteRadarValueListItemsItemRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, DeleteRadarValueListItemsItemRequest> {
    type Output = httpclient::InMemoryResult<DeletedRadarValueListItem>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/radar/value_list_items/{item}", item = self.params.item
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}