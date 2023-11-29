//! [`StripeClient`](struct.StripeClient.html) is the main entry point for this library.
//!
//! Library created with [`libninja`](https://www.libninja.com).
#![allow(non_camel_case_types)]
#![allow(unused)]
pub mod model;
pub mod request;
pub use httpclient::{Error, Result, InMemoryResponseExt};
use crate::model::*;
impl StripeClient {
    pub fn from_env() -> Self {
        Self {
            client: httpclient::Client::new().base_url("https://api.stripe.com/"),
            authentication: StripeAuthentication::from_env(),
        }
    }
}
impl StripeAuthentication {
    pub fn from_env() -> Self {
        Self::BasicAuth {
            basic_auth: {
                let mut value = std::env::var("STRIPE_SECRET_KEY")
                    .expect("Environment variable BASIC_AUTH is not set.");
                value.push_str(":");
                STANDARD_NO_PAD.encode(value)
            },
        }
    }
}