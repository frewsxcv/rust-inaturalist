# iNaturalist OAuth

This crate provides a simple way to obtain an iNaturalist API token using the OAuth2 authorization flow.

## Overview

This crate supports different levels of control over the OAuth2 flow, catering to various use cases from simple CLI tools to more complex web applications.

1.  **Simple Flow (for CLI tools):** The `get_api_token` method handles the entire process automatically: it opens the user's browser, runs a temporary local server to catch the redirect, and exchanges the authorization code for an API token. This is the easiest way to get started.

2.  **Advanced Flow (for Web Applications):** For scenarios where you can't open a browser or run a local server (like in a web app), you can use a two-step process. First, generate the authorization URL with `authorization_url()` to redirect the user. Second, after the user authorizes and is redirected back to your service, use `exchange_code()` to get the API token.

3.  **Custom Flow (Granular Control):** You can also control each step of the process individually. This is useful if you want to, for example, open the browser yourself or handle the local server listening in a separate thread. This involves using `authorization_url()`, `listen_for_redirect()`, and `exchange_code()` as separate steps.

The primary entry point for all flows is the `Authenticator`.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
inaturalist-oauth = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

### Simple Usage (for CLI tools)

The `get_api_token()` method handles the entire process automatically.

```rust
use inaturalist_oauth::Authenticator;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client_id = "YOUR_CLIENT_ID".to_string();
    let client_secret = "YOUR_CLIENT_SECRET".to_string();

    let token_details = Authenticator::new(client_id, client_secret)
        .with_redirect_server_port(8081)
        .get_api_token()
        .await?;

    println!("Got iNaturalist API token: {}", token_details.api_token);
    println!("Token expires at: {:?}", token_details.expires_at);

    // Now you can use this token to make authenticated requests to the iNaturalist API.
    Ok(())
}
```

### Advanced Usage (for Web Applications)

For web applications, you need to handle redirects manually. This requires a two-step process.

**Step 1: Get the Authorization URL and Redirect**

First, create an `Authenticator` and call `authorization_url()`. This will give you the URL to redirect the user to and a `pkce_verifier` that you must store (e.g., in the user's session) to use in the next step.

```rust
# use inaturalist_oauth::{Authenticator, AuthorizationInfo};
# use std::error::Error;
#
# fn get_redirect_url() -> Result<(), Box<dyn Error>> {
let authenticator = Authenticator::new(
    "YOUR_CLIENT_ID".to_string(),
    "YOUR_CLIENT_SECRET".to_string(),
)
.with_redirect_server_port(8000); // Should match your app's redirect URI port

let auth_info = authenticator.authorization_url()?;

// 1. Store `auth_info.pkce_verifier` in the user's session.
// 2. Redirect the user to `auth_info.url`.
# Ok(())
# }
```

**Step 2: Handle the Redirect and Exchange the Code for a Token**

After the user authorizes your application, iNaturalist will redirect them back to your application's redirect URI. The URL will contain an authorization `code`. Retrieve this code, along with the `pkce_verifier` you stored in the session, and call `exchange_code()` to get the API token.

```rust
# use inaturalist_oauth::{Authenticator, PkceVerifier};
# use oauth2::AuthorizationCode;
# use std::error::Error;
#
# async fn handle_redirect() -> Result<(), Box<dyn Error>> {
# let authenticator = Authenticator::new("".into(), "".into());
// 1. Retrieve the `code` from the request's query parameters.
let code = AuthorizationCode::new("...code from query params...".to_string());

// 2. Retrieve the `pkce_verifier` you stored in the user's session.
let pkce_verifier = PkceVerifier::new("...verifier from session...".to_string());

// 3. Exchange the code for a token.
let token_details = authenticator.exchange_code(code, pkce_verifier).await?;

println!("Got iNaturalist API token: {}", token_details.api_token);

// 4. Store the token details for future use.
# Ok(())
# }
```

### Custom Flow (Granular Control)

If you need more control over the CLI process, you can use the individual methods to orchestrate the flow yourself. This gives you direct access to each step.

```rust
use inaturalist_oauth::Authenticator;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let authenticator = Authenticator::new(
        "YOUR_CLIENT_ID".to_string(),
        "YOUR_CLIENT_SECRET".to_string(),
    );

    // 1. Get the authorization URL and PKCE verifier.
    let auth_info = authenticator.authorization_url()?;
    let pkce_verifier = auth_info.pkce_verifier;

    // 2. Open the browser for the user to authorize.
    opener::open(auth_info.url.to_string())?;
    println!("Please authorize the application in your browser.");

    // 3. Listen for the redirect. This is a blocking operation that starts a
    //    local server and waits for the user to be redirected back.
    let code = authenticator.listen_for_redirect()?;

    // 4. Exchange the authorization code for an API token.
    let token_details = authenticator.exchange_code(code, pkce_verifier).await?;

    println!("Successfully obtained API token: {}", token_details.api_token);
    Ok(())
}
```
