use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let shipping_rate_token = "your shipping rate token";
    let response = client
        .post_shipping_rates_shipping_rate_token(shipping_rate_token)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
