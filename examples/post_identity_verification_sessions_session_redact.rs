#![allow(unused_imports)]
use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let session = "your session";
    let response = client
        .post_identity_verification_sessions_session_redact(session)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
