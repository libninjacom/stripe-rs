use serde_json::json;
use crate::model::*;
use crate::StripeClient;
use httpclient::InMemoryResponseExt;
/**Create this with the associated client method.

That method takes required values as arguments. Set optional values using builder methods on this struct.*/
#[derive(Clone)]
pub struct PostInvoicesInvoiceLinesLineItemIdRequest<'a> {
    pub(crate) http_client: &'a StripeClient,
    pub invoice: String,
    pub line_item_id: String,
}
impl<'a> PostInvoicesInvoiceLinesLineItemIdRequest<'a> {
    pub async fn send(self) -> ::httpclient::InMemoryResult<LineItem> {
        let mut r = self
            .http_client
            .client
            .post(
                &format!(
                    "/v1/invoices/{invoice}/lines/{line_item_id}", invoice = self
                    .invoice, line_item_id = self.line_item_id
                ),
            );
        r = self.http_client.authenticate(r);
        let res = r.await?;
        res.json().map_err(Into::into)
    }
}
impl<'a> ::std::future::IntoFuture for PostInvoicesInvoiceLinesLineItemIdRequest<'a> {
    type Output = httpclient::InMemoryResult<LineItem>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(self.send())
    }
}