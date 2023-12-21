use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::post_file_links_link`].

On request success, this will return a [`FileLink`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PostFileLinksLinkRequest {
    pub link: String,
}
impl PostFileLinksLinkRequest {}
impl FluentRequest<'_, PostFileLinksLinkRequest> {}
impl<'a> ::std::future::IntoFuture for FluentRequest<'a, PostFileLinksLinkRequest> {
    type Output = httpclient::InMemoryResult<FileLink>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!("/v1/file_links/{link}", link = self.params.link);
            let mut r = self.client.client.post(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}