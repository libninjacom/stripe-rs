#![allow(unused_imports)]
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let source = "your source";
    let source_transaction = "your source transaction";
    let response = client
        .get_sources_source_source_transactions_source_transaction(
            source,
            source_transaction,
        )
        .expand(&["your expand"])
        .await
        .unwrap();
    println!("{:#?}", response);
}