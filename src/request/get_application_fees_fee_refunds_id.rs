use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_application_fees_fee_refunds_id`].

On request success, this will return a [`FeeRefund`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetApplicationFeesFeeRefundsIdRequest {
    pub expand: Option<Vec<String>>,
    pub fee: String,
    pub id: String,
}
impl GetApplicationFeesFeeRefundsIdRequest {}
impl FluentRequest<'_, GetApplicationFeesFeeRefundsIdRequest> {
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetApplicationFeesFeeRefundsIdRequest> {
    type Output = httpclient::InMemoryResult<FeeRefund>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/application_fees/{fee}/refunds/{id}", fee = self.params.fee, id =
                self.params.id
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}