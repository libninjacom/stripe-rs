use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_coupons_coupon`].

On request success, this will return a [`Coupon`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostCouponsCouponRequest {
    pub coupon: String,
}
impl PostCouponsCouponRequest {}
impl FluentRequest<'_, PostCouponsCouponRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostCouponsCouponRequest> {
    type Output = httpclient::InMemoryResult<Coupon>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/coupons/{coupon}", coupon = self.params.coupon);
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}