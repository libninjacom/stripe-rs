use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_radar_value_lists`].

On request success, this will return a [`RadarValueList`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRadarValueListsRequest {}
impl PostRadarValueListsRequest {}
impl FluentRequest<'_, PostRadarValueListsRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostRadarValueListsRequest> {
    type Output = httpclient::InMemoryResult<RadarValueList>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = "/v1/radar/value_lists";
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}