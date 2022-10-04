use stripe2::StripeClient;
use stripe2::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let location = "your location";
    let response = client
        .delete_terminal_locations_location(location)
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
