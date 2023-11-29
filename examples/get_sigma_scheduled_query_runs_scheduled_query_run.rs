#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let scheduled_query_run = "your scheduled query run";
    let response = client
        .get_sigma_scheduled_query_runs_scheduled_query_run(scheduled_query_run)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}