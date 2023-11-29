use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostCouponsRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
}
impl<'a> PostCouponsRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Coupon> {
        let mut r = self.http_client.client.post("/v1/coupons");
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostCouponsRequest<'a> {
    type Output = httpclient::InMemoryResult<Coupon>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}