use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let name = "your name";
    let scope = ::serde_json::json!({});
    let response = client
        .get_apps_secrets_find(name, scope)
        .expand(&["your expand"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
