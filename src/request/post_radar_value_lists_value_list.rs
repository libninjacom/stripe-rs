use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostRadarValueListsValueListRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub value_list: String,
}
impl<'a> PostRadarValueListsValueListRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<RadarValueList> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v1/radar/value_lists/{value_list}", value_list = self.value_list
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostRadarValueListsValueListRequest<'a> {
    type Output = httpclient::InMemoryResult<RadarValueList>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}