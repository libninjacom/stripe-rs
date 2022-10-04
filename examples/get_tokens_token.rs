use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let token = "your token";
    let response = client
        .get_tokens_token(token)
        .expand(&["your expand"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
