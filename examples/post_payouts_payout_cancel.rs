use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let payout = "your payout";
    let response = client.post_payouts_payout_cancel(payout).send().await.unwrap();
    println!("{:#?}", response);
}
