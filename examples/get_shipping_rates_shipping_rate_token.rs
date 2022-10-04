use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let shipping_rate_token = "your shipping rate token";
    let response = client
        .get_shipping_rates_shipping_rate_token(shipping_rate_token)
        .expand(&["your expand"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
