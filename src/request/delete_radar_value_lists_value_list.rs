use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct DeleteRadarValueListsValueListRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub value_list: String,
}
impl<'a> DeleteRadarValueListsValueListRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<DeletedRadarValueList> {
        let mut r = self
            .http_client
            .client
            .delete(
                &format!(
                    "/v1/radar/value_lists/{value_list}", value_list = self.value_list
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for DeleteRadarValueListsValueListRequest<'a> {
    type Output = httpclient::InMemoryResult<DeletedRadarValueList>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}