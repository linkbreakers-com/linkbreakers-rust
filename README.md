# Linkbreakers Rust SDK

Official Rust SDK for the Linkbreakers API, auto-generated from OpenAPI specification.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
linkbreakers = "1"
tokio = { version = "1", features = ["full"] }
```

## Requirements

- Rust 1.70 or higher

## Usage

### Async Example

```rust
use linkbreakers::{apis::configuration::Configuration, apis::links_api, models::CreateLinkRequest};

#[tokio::main]
async fn main() {
    // Create configuration with authentication
    let mut config = Configuration::new();
    config.bearer_access_token = Some("your-api-token".to_string());

    // Create a shortened link
    let mut request = CreateLinkRequest::new("https://example.com".to_string());
    request.name = Some("My Link".to_string());

    match links_api::links_service_create(&config, request, None).await {
        Ok(response) => {
            let shortlink = response.link.and_then(|link| link.shortlink).unwrap_or_default();
            println!("Short link: {}", shortlink);
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }

    // List links: page_size, then the optional page_token, include, tags,
    // search, sort_by, sort_direction, view, directory_id and cursor
    match links_api::links_service_list(&config, 10, None, None, None, None, None, None, None, None, None).await {
        Ok(response) => {
            println!("Found {} links", response.links.unwrap_or_default().len());
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
```

Each API is a module under `linkbreakers::apis` (`links_api`, `visitors_api`, ...) and each operation is a function named after the API's operation ID (`links_service_create`, `links_service_list`, `visitors_service_identify`, ...). It takes the configuration, then the path parameters, request body and query parameters in order, with optional ones as `Option`.

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
        links_api::links_service_list(&config, 10, None, None, None, None, None, None, None, None, None).await
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

This SDK is automatically generated from the Linkbreakers OpenAPI specification and published when the API is updated. This README is written by hand: `scripts/check-docs.sh` compiles every Rust block in it against the generated code, on pull requests and before every release.

Current API version: See [OPENAPI_VERSION](./OPENAPI_VERSION)

## Issues

Report issues at: https://github.com/linkbreakers-com/linkbreakers-rust/issues

## License

MIT License
