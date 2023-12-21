use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_terminal_readers_reader_set_reader_display`].

On request success, this will return a [`TerminalReader`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTerminalReadersReaderSetReaderDisplayRequest {
    pub reader: String,
}
impl PostTerminalReadersReaderSetReaderDisplayRequest {}
impl FluentRequest<'_, PostTerminalReadersReaderSetReaderDisplayRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostTerminalReadersReaderSetReaderDisplayRequest> {
    type Output = httpclient::InMemoryResult<TerminalReader>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/terminal/readers/{reader}/set_reader_display", reader = self.params
                .reader
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}