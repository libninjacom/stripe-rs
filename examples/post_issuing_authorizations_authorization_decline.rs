use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let authorization = "your authorization";
    let response = client
        .post_issuing_authorizations_authorization_decline(authorization)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
