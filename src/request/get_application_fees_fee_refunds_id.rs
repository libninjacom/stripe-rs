use serde_json::json;
use crate::model::*;
use crate::StripeClient;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct GetApplicationFeesFeeRefundsIdRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub expand: Option<Vec<String>>,
    pub fee: String,
    pub id: String,
}
impl<'a> GetApplicationFeesFeeRefundsIdRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<FeeRefund> {
        let mut r = self
            .http_client
            .client
            .get(
                &format!(
                    "/v1/application_fees/{fee}/refunds/{id}", fee = self.fee, id = self
                    .id
                ),
            );
        if let Some(ref unwrapped) = self.expand {
            for item in unwrapped {
                r = r.query("expand[]", &item.to_string());
            }
        }
        r = self.http_client.authenticate(r);
        let res = r.send_awaiting_body().await?;
        res.json().map_err(Into::into)
    }
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self.expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture for GetApplicationFeesFeeRefundsIdRequest<'a> {
    type Output = httpclient::InMemoryResult<FeeRefund>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}