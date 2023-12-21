use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_promotion_codes_promotion_code`].

On request success, this will return a [`PromotionCode`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostPromotionCodesPromotionCodeRequest {
    pub promotion_code: String,
}
impl PostPromotionCodesPromotionCodeRequest {}
impl FluentRequest<'_, PostPromotionCodesPromotionCodeRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostPromotionCodesPromotionCodeRequest> {
    type Output = httpclient::InMemoryResult<PromotionCode>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/promotion_codes/{promotion_code}", promotion_code = self.params
                .promotion_code
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}