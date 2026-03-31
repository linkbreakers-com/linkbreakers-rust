# Linkbreakers Rust SDK

Official Rust SDK for the Linkbreakers API, auto-generated from OpenAPI specification.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
linkbreakers = "1.32.0"
tokio = { version = "1", features = ["full"] }
```

## Requirements

- Rust 1.70 or higher

## Usage

### Async Example

```rust
use linkbreakers::{apis::configuration::Configuration, apis::links_api};

#[tokio::main]
async fn main() {
    // Create configuration with authentication
    let mut config = Configuration::new();
    config.bearer_access_token = Some("your-api-token".to_string());

    // Example: List links
    match links_api::links_list(&config, Some(10), None, None, None, None, None, None, None, None).await {
        Ok(response) => {
            println!("Found {} links", response.links.unwrap_or_default().len());
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

### Blocking Example

```rust
use linkbreakers::{apis::configuration::Configuration, apis::links_api};

fn main() {
    // Create configuration with authentication
    let mut config = Configuration::new();
    config.bearer_access_token = Some("your-api-token".to_string());

    // Example: List links (blocking)
    let runtime = tokio::runtime::Runtime::new().unwrap();
    let result = runtime.block_on(async {
        links_api::links_list(&config, Some(10), None, None, None, None, None, None, None, None).await
    });

    match result {
        Ok(response) => {
            println!("Found {} links", response.links.unwrap_or_default().len());
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

## Authentication

The SDK supports Bearer token authentication:

```rust
use linkbreakers::apis::configuration::Configuration;

let mut config = Configuration::new();
config.bearer_access_token = Some("your-api-token".to_string());

// Use config with API calls
```

## API Documentation

For detailed API documentation, visit: https://docs.linkbreakers.com

For Rust-specific docs, run: `cargo doc --open`

## Auto-Generated

This SDK is automatically generated from the Linkbreakers OpenAPI specification and published when the API is updated.

Current API version: See [OPENAPI_VERSION](./OPENAPI_VERSION)

## Issues

Report issues at: https://github.com/linkbreakers-com/linkbreakers-rust/issues

## License

MIT License
