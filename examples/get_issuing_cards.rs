use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client
        .get_issuing_cards()
        .cardholder("your cardholder")
        .created(::serde_json::json!({}))
        .ending_before("your ending before")
        .exp_month(1)
        .exp_year(1)
        .expand(&["your expand"])
        .last4("your last 4")
        .limit(1)
        .starting_after("your starting after")
        .status("your status")
        .type_("your type")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
