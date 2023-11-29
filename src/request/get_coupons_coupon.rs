use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetCouponsCouponRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub coupon: String,
    pub expand: Option<Vec<String>>,
}
impl<'a> GetCouponsCouponRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<Coupon> {
        let mut r = self
            .http_client
            .client
            .get(&format!("/v1/coupons/{coupon}", coupon = self.coupon));
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.query("expand[]", &item.to_string());
            }
        }
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetCouponsCouponRequest<'a> {
    type Output = httpclient::InMemoryResult<Coupon>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}