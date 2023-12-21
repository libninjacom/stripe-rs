use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_terminal_readers_reader_process_setup_intent`].

On request success, this will return a [`TerminalReader`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostTerminalReadersReaderProcessSetupIntentRequest {
    pub reader: String,
}
impl PostTerminalReadersReaderProcessSetupIntentRequest {}
impl FluentRequest<'_, PostTerminalReadersReaderProcessSetupIntentRequest> {}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, PostTerminalReadersReaderProcessSetupIntentRequest> {
    type Output = httpclient::InMemoryResult<TerminalReader>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/terminal/readers/{reader}/process_setup_intent", reader = self
                .params.reader
            );
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}