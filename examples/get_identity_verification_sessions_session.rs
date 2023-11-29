#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let session = "your session";
    let response = client
        .get_identity_verification_sessions_session(session)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}