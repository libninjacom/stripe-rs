use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let price = "your price";
    let response = client.post_prices_price(price).send().await.unwrap();
    println!("{:#?}", response);
}
