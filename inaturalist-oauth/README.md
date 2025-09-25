# iNaturalist OAuth

This crate provides a simple way to obtain an iNaturalist API token using the OAuth2 authorization flow.

## Overview

The primary entry point is the `Authenticator`. You use it to configure your application's client ID and secret, and then call `get_api_token()`. When called, it will:

1.  It initiates the OAuth2 flow.
2.  It opens the user's default web browser to the iNaturalist authorization page.
3.  The user must then log in to iNaturalist (if not already) and authorize the application.
4.  After authorization, iNaturalist redirects the user to a temporary local web server (defaults to `http://localhost:8080`, but is configurable).
5.  This crate captures the authorization code from the redirect.
6.  It exchanges the authorization code for an OAuth2 access token.
7.  It uses the access token to request a long-lived iNaturalist API token.
8.  The new API token is returned.

This process provides a user-friendly way for a command-line application to get authorization to access the iNaturalist API on behalf of a user. The calling application is responsible for storing and retrieving the token for subsequent use.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
inaturalist-oauth = { path = "../inaturalist-oauth" }
```

Then, you can use it in your code like this:

```rust
use inaturalist_oauth::Authenticator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = "YOUR_CLIENT_ID".to_string();
    let client_secret = "YOUR_CLIENT_SECRET".to_string();

    let api_token = Authenticator::new(client_id, client_secret)
        .with_redirect_server_port(8081)
        .get_api_token()?;
    println!("Got iNaturalist API token: {}", api_token);
    // Now you can use this token to make authenticated requests to the iNaturalist API.
    Ok(())
}
```
