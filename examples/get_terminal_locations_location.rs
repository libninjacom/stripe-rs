use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let location = "your location";
    let response = client
        .get_terminal_locations_location(location)
        .expand(&["your expand"])
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
