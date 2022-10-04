use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let plan = "your plan";
    let response = client.post_plans_plan(plan).send().await.unwrap();
    println!("{:#?}", response);
}
