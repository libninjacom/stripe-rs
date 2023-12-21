use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_radar_value_lists_value_list`].

On request success, this will return a [`RadarValueList`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostRadarValueListsValueListRequest {
    pub value_list: String,
}
impl PostRadarValueListsValueListRequest {}
impl FluentRequest<'_, PostRadarValueListsValueListRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostRadarValueListsValueListRequest> {
    type Output = httpclient::InMemoryResult<RadarValueList>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/radar/value_lists/{value_list}", value_list = self.params.value_list
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}