<div id="top"></div>

<p align="center">
    <a href="https://github.com/libninjacom/stripe-rs/stargazers">
        <img src="https://img.shields.io/github/stars/libninjacom/stripe-rs.svg?style=flat-square" alt="Stars" />
    </a>
    <a href="https://github.com/libninjacom/stripe-rs/actions">
        <img src="https://img.shields.io/github/workflow/status/libninjacom/stripe-rs/ci?style=flat-square" alt="Build Status" />
    </a>
    
<a href="https://crates.io/crates/stripe2">
    <img src="https://img.shields.io/crates/d/stripe2?style=flat-square" alt="Downloads" />
</a>
<a href="https://crates.io/crates/stripe2">
    <img src="https://img.shields.io/crates/v/stripe2?style=flat-square" alt="Crates.io" />
</a>

</p>

Stripe client, generated from the OpenAPI spec.

# Usage

```rust
use stripe::StripeClient;
use stripe::model::*;
#[tokio::main]
async fn main() {
    let client = StripeClient::from_env();
    let response = client.get_account().await.unwrap();
    println!("{:#?}", response);
}
```

This example loads configuration from environment variables, specifically:

* `STRIPE_SECRET_KEY`

# Installation

Add this to your Cargo.toml:

```toml
[dependencies]
stripe2 = ".."
```


# Documentation

* [Client Library Documentation](https://docs.rs/stripe2)

You can see working examples of every API call in the `examples/` directory.

# Contributing

Contributions are welcome!

*Library created with [Libninja](https://www.libninja.com).*
