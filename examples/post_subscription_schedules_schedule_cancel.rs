#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let schedule = "your schedule";
    let response = client
        .post_subscription_schedules_schedule_cancel(schedule)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
