use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::delete_radar_value_lists_value_list`].

On request success, this will return a [`DeletedRadarValueList`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRadarValueListsValueListRequest {
    pub value_list: String,
}
impl DeleteRadarValueListsValueListRequest {}
impl FluentRequest<'_, DeleteRadarValueListsValueListRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, DeleteRadarValueListsValueListRequest> {
    type Output = httpclient::InMemoryResult<DeletedRadarValueList>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/radar/value_lists/{value_list}", value_list = self.params.value_list
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}