#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let plan = "your plan";
    let response = client.get_plans_plan(plan).expand(&["your expand"]).await.unwrap();
    println!("{:#?}", response);
}