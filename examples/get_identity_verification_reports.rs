use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_identity_verification_reports()
        .created(::serde_json::json!({}))
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .starting_after("your starting after")
        .type_("your type")
        .verification_session("your verification session")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
