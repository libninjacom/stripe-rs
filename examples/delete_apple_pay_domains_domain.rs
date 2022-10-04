use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let domain = "your domain";
    let response = client.delete_apple_pay_domains_domain(domain).send().await.unwrap();
    println!("{:#?}", response);
}
