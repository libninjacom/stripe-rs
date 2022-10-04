use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let cardholder = "your cardholder";
    let response = client
        .post_issuing_cardholders_cardholder(cardholder)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
