use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::delete_invoices_invoice`].

On request success, this will return a [`DeletedInvoice`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteInvoicesInvoiceRequest {
    pub invoice: String,
}
impl DeleteInvoicesInvoiceRequest {}
impl FluentRequest<'_, DeleteInvoicesInvoiceRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, DeleteInvoicesInvoiceRequest> {
    type Output = httpclient::InMemoryResult<DeletedInvoice>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/invoices/{invoice}", invoice = self.params.invoice);
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}