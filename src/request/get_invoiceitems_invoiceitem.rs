use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_invoiceitems_invoiceitem`].

On request success, this will return a [`Invoiceitem`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetInvoiceitemsInvoiceitemRequest {
    pub expand: Option<Vec<String>>,
    pub invoiceitem: String,
}
impl GetInvoiceitemsInvoiceitemRequest {}
impl FluentRequest<'_, GetInvoiceitemsInvoiceitemRequest> {
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetInvoiceitemsInvoiceitemRequest> {
    type Output = httpclient::InMemoryResult<Invoiceitem>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/invoiceitems/{invoiceitem}", invoiceitem = self.params.invoiceitem
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}