use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_events()
        .created(::serde_json::json!({}))
        .delivery_success(true)
        .ending_before("your ending before")
        .expand(&["your expand"])
        .limit(1)
        .starting_after("your starting after")
        .type_("your type")
        .types(&["your types"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
