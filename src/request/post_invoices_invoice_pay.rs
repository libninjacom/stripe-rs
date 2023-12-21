use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_invoices_invoice_pay`].

On request success, this will return a [`Invoice`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostInvoicesInvoicePayRequest {
    pub invoice: String,
}
impl PostInvoicesInvoicePayRequest {}
impl FluentRequest<'_, PostInvoicesInvoicePayRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostInvoicesInvoicePayRequest> {
    type Output = httpclient::InMemoryResult<Invoice>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/invoices/{invoice}/pay", invoice = self.params.invoice
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}