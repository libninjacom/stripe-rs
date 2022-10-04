use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let source = "your source";
    let response = client.post_sources_source_verify(source).send().await.unwrap();
    println!("{:#?}", response);
}
