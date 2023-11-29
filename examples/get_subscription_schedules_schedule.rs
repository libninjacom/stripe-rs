#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let schedule = "your schedule";
    let response = client
        .get_subscription_schedules_schedule(schedule)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}