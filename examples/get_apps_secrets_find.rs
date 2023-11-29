#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let name = "your name";
    let scope = ScopeParam {
        type_: "your type".to_owned(),
        user: Some("your user".to_owned()),
    };
    let response = client
        .get_apps_secrets_find(name, scope)
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}