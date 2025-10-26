# Mailpit REST API Client
[![Crates.io](https://img.shields.io/crates/v/mailpit_client.svg)](https://crates.io/crates/mailpit_client)
[![license](https://img.shields.io/badge/license-Apache-blue.svg)](./LICENSE)
[![license](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)

## Features
Implements [Mailpit REST API version 1.27.10](https://mailpit.axllent.org/docs/api-v1/).

## Usage
Copy this to your Cargo.toml

```toml
mailpit_client = "0.1.0"
```

and use it like this

```rust
use mailpit_client::{MailpitClient, models::*};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = MailpitClient::new("http://localhost:8025/")?;

    let app_info = client.get_application_information().await?;
    println!("{app_info:#?}");

    Ok(())
}
```
