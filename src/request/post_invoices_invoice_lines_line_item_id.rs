use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_invoices_invoice_lines_line_item_id`].

On request success, this will return a [`LineItem`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostInvoicesInvoiceLinesLineItemIdRequest {
    pub invoice: String,
    pub line_item_id: String,
}
impl PostInvoicesInvoiceLinesLineItemIdRequest {}
impl FluentRequest<'_, PostInvoicesInvoiceLinesLineItemIdRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostInvoicesInvoiceLinesLineItemIdRequest> {
    type Output = httpclient::InMemoryResult<LineItem>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/invoices/{invoice}/lines/{line_item_id}", invoice = self.params
                .invoice, line_item_id = self.params.line_item_id
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}