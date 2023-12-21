use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_radar_value_list_items_item`].

On request success, this will return a [`RadarValueListItem`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetRadarValueListItemsItemRequest {
    pub expand: Option<Vec<String>>,
    pub item: String,
}
impl GetRadarValueListItemsItemRequest {}
impl FluentRequest<'_, GetRadarValueListItemsItemRequest> {
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetRadarValueListItemsItemRequest> {
    type Output = httpclient::InMemoryResult<RadarValueListItem>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/radar/value_list_items/{item}", item = self.params.item
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}