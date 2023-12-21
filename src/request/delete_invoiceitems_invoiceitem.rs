use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::delete_invoiceitems_invoiceitem`].

On request success, this will return a [`DeletedInvoiceitem`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteInvoiceitemsInvoiceitemRequest {
    pub invoiceitem: String,
}
impl DeleteInvoiceitemsInvoiceitemRequest {}
impl FluentRequest<'_, DeleteInvoiceitemsInvoiceitemRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, DeleteInvoiceitemsInvoiceitemRequest> {
    type Output = httpclient::InMemoryResult<DeletedInvoiceitem>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/invoiceitems/{invoiceitem}", invoiceitem = self.params.invoiceitem
            );
            let mut r = self.client.client.delete(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}