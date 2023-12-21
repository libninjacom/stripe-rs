use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_radar_value_list_items`].

On request success, this will return a [`RadarValueListItem`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRadarValueListItemsRequest {}
impl PostRadarValueListItemsRequest {}
impl FluentRequest<'_, PostRadarValueListItemsRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostRadarValueListItemsRequest> {
    type Output = httpclient::InMemoryResult<RadarValueListItem>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/radar/value_list_items";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}