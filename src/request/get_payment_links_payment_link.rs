use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_payment_links_payment_link`].

On request success, this will return a [`PaymentLink`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetPaymentLinksPaymentLinkRequest {
    pub expand: Option<Vec<String>>,
    pub payment_link: String,
}
impl GetPaymentLinksPaymentLinkRequest {}
impl FluentRequest<'_, GetPaymentLinksPaymentLinkRequest> {
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetPaymentLinksPaymentLinkRequest> {
    type Output = httpclient::InMemoryResult<PaymentLink>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/payment_links/{payment_link}", payment_link = self.params
                .payment_link
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}