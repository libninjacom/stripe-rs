use serde_json::json;
use crate::model::*;
use crate::FluentRequest;
use serde::{Serialize, Deserialize};
use httpclient::InMemoryResponseExt;
use crate::StripeClient;
/**You should use this struct via [`StripeClient::get_sigma_scheduled_query_runs_scheduled_query_run`].

On request success, this will return a [`ScheduledQueryRun`].*/
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetSigmaScheduledQueryRunsScheduledQueryRunRequest {
    pub expand: Option<Vec<String>>,
    pub scheduled_query_run: String,
}
impl GetSigmaScheduledQueryRunsScheduledQueryRunRequest {}
impl FluentRequest<'_, GetSigmaScheduledQueryRunsScheduledQueryRunRequest> {
    pub fn expand(mut self, expand: impl IntoIterator<Item = impl AsRef<str>>) -> Self {
        self
            .params
            .expand = Some(expand.into_iter().map(|s| s.as_ref().to_owned()).collect());
        self
    }
}
impl<'a> ::std::future::IntoFuture
for FluentRequest<'a, GetSigmaScheduledQueryRunsScheduledQueryRunRequest> {
    type Output = httpclient::InMemoryResult<ScheduledQueryRun>;
    type IntoFuture = ::futures::future::BoxFuture<'a, Self::Output>;
    fn into_future(self) -> Self::IntoFuture {
        Box::pin(async move {
            let url = &format!(
                "/v1/sigma/scheduled_query_runs/{scheduled_query_run}",
                scheduled_query_run = self.params.scheduled_query_run
            );
            let mut r = self.client.client.get(url);
            r = r.set_query(self.params);
            r = self.client.authenticate(r);
            let res = r.await?;
            res.json().map_err(Into::into)
        })
    }
}