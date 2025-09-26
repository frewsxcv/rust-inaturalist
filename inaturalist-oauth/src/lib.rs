//! This crate provides a simple way to obtain an iNaturalist API token using the OAuth2
//! authorization flow. It handles the process of opening a web browser for user
//! authorization, running a temporary local server to catch the redirect, and
//! exchanging the authorization code for an API token.
//!
//! ## Usage
//!
//! ```no_run
//! use inaturalist_oauth::Authenticator;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client_id = "YOUR_CLIENT_ID".to_string();
//!     let client_secret = "YOUR_CLIENT_SECRET".to_string();
//!
//!     let token_details = Authenticator::new(client_id, client_secret)
//!         .with_redirect_server_port(8081)
//!         .get_api_token()
//!         .await?;
//!     println!("Got iNaturalist API token: {}", token_details.api_token);
//!     Ok(())
//! }
//! ```
use oauth2::basic::BasicClient;
use oauth2::http::{HeaderMap, HeaderValue, Method};
use oauth2::{
    AuthUrl, AuthorizationCode, ClientId, ClientSecret, CsrfToken, PkceCodeChallenge, RedirectUrl,
    TokenResponse, TokenUrl,
};
use serde::{Deserialize, Serialize};
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::time::{Duration, SystemTime};
use url::Url;

/// Contains the iNaturalist API token and its expiration details.
///
/// While the iNaturalist API token itself doesn't expire, the OAuth2 access token
/// used to obtain it does (typically after 24 hours). This struct includes an
/// `expires_at` field to indicate when the original access token expires.
/// This can be useful for prompting the user to re-authenticate if you want to
/// ensure the entire authentication flow is periodically re-validated.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenDetails {
    /// The long-lived iNaturalist API token.
    pub api_token: String,
    /// The time when the underlying OAuth2 access token expires.
    pub expires_at: SystemTime,
}

/// Handles the iNaturalist OAuth2 flow to obtain an API token.
///
/// This struct is used to configure the authenticator and initiate the OAuth2 flow.
pub struct Authenticator {
    client_id: ClientId,
    client_secret: ClientSecret,
    port: u16,
}

impl Authenticator {
    /// Creates a new `Authenticator`.
    ///
    /// # Arguments
    ///
    /// * `client_id` - The iNaturalist application's client ID.
    /// * `client_secret` - The iNaturalist application's client secret.
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client_id: ClientId::new(client_id),
            client_secret: ClientSecret::new(client_secret),
            port: 8080,
        }
    }

    /// Sets the port for the local redirect server.
    ///
    /// The default port is 8080.
    pub fn with_redirect_server_port(mut self, port: u16) -> Self {
        self.port = port;
        self
    }

    /// Initiates the OAuth2 flow to get an iNaturalist API token.
    ///
    /// This method opens the user's web browser to the iNaturalist authorization page.
    /// After the user authorizes the application, it completes the OAuth2 flow,
    /// obtains an access token, and then exchanges it for a long-lived API token.
    pub async fn get_api_token(self) -> Result<TokenDetails, Box<dyn std::error::Error>> {
        // Set up a local TCP listener to act as the redirect URI.
        let listener = TcpListener::bind(("127.0.0.1", self.port))?;
        let port = listener.local_addr()?.port();
        let redirect_url = format!("http://localhost:{}", port);
        let client = self.client(&redirect_url)?;

        // 1. Generate a PKCE (Proof Key for Code Exchange) challenge to secure the flow.
        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        // 2. Construct the authorization URL that the user will visit.
        let (auth_url, _csrf_token) = client
            .authorize_url(CsrfToken::new_random)
            .set_pkce_challenge(pkce_challenge)
            .url();

        // 3. Open the URL in the user's default web browser.
        log::info!("Opening browser to: {auth_url}");
        opener::open(auth_url.to_string())?;

        // 4. Listen for the redirect from iNaturalist, which will contain the authorization code.
        //    This is a blocking call that will wait until the user authorizes the app.
        let code = self.listen_for_code(listener)?;

        // 5. Exchange the authorization code for an OAuth2 access token.
        let token_response = client
            .exchange_code(code)
            .set_pkce_verifier(pkce_verifier)
            .request_async(oauth2::reqwest::async_http_client)
            .await?;

        let token_string = token_response.access_token().secret();
        log::info!("OAuth access token: {token_string}");

        // 6. Use the access token to request a long-lived iNaturalist API token.
        let response = self.fetch_api_token(token_string).await?;
        log::info!("OAuth API token: {}", response.api_token);

        // 7. Return the API token and calculated expiration time.
        let expires_at = SystemTime::now() + Duration::from_secs(24 * 60 * 60);
        Ok(TokenDetails {
            api_token: response.api_token,
            expires_at,
        })
    }

    /// Listens for and handles incoming HTTP requests to extract the authorization code.
    fn listen_for_code(
        &self,
        listener: TcpListener,
    ) -> Result<AuthorizationCode, Box<dyn std::error::Error>> {
        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    // Attempt to handle the connection. If we get a code, we're done.
                    if let Ok(code) = self.handle_connection(stream) {
                        return Ok(code);
                    }
                    // If handle_connection failed, an error response was already sent
                    // to the browser. We can continue listening for another request
                    // (e.g., from browser retries or favicon requests).
                }
                Err(e) => {
                    log::error!("Failed to accept connection: {e}");
                }
            }
        }
        Err("Server closed before receiving authorization code".into())
    }

    /// Handles a single incoming TCP connection, parsing the request and sending a response.
    fn handle_connection(
        &self,
        mut stream: TcpStream,
    ) -> Result<AuthorizationCode, Box<dyn std::error::Error>> {
        let mut buffer = [0; 1024];
        stream.read(&mut buffer)?;

        match self.parse_code_from_request(&buffer) {
            Ok(code) => {
                let message = "<h1>Success!</h1><p>You can close this window now.</p>";
                self.send_response(&mut stream, "200 OK", message)?;
                Ok(code)
            }
            Err(e) => {
                log::error!("Failed to parse code from request: {e}");
                let message = "<h1>Error!</h1><p>Could not get authorization code from iNaturalist. Please try again.</p>";
                self.send_response(&mut stream, "400 Bad Request", message)?;
                Err(e)
            }
        }
    }

    /// Parses an HTTP request from the raw byte buffer to find the `code` query parameter.
    fn parse_code_from_request(
        &self,
        buffer: &[u8],
    ) -> Result<AuthorizationCode, Box<dyn std::error::Error>> {
        let mut headers = [httparse::EMPTY_HEADER; 64];
        let mut req = httparse::Request::new(&mut headers);
        req.parse(buffer)?;

        let path = req.path.ok_or("Malformed request: no path")?;
        let url = Url::parse(&format!("http://localhost{path}"))?;

        let code_pair = url
            .query_pairs()
            .find(|pair| pair.0 == "code")
            .ok_or_else(|| format!("URL did not contain 'code' parameter: {url}"))?;

        Ok(AuthorizationCode::new(code_pair.1.into_owned()))
    }

    /// Writes a simple HTTP response to the given stream.
    fn send_response(
        &self,
        stream: &mut TcpStream,
        status: &str,
        body: &str,
    ) -> std::io::Result<()> {
        let response = format!(
            "HTTP/1.1 {}\r\ncontent-length: {}\r\n\r\n{}",
            status,
            body.len(),
            body
        );
        stream.write_all(response.as_bytes())
    }

    /// Fetches the long-lived iNaturalist API token using the OAuth2 access token.
    async fn fetch_api_token(
        &self,
        token_string: &str,
    ) -> Result<ApiTokenResponse, Box<dyn std::error::Error>> {
        let mut headers = HeaderMap::new();
        headers.append(
            "Authorization",
            HeaderValue::from_str(&format!("Bearer {token_string}"))?,
        );
        let request = oauth2::HttpRequest {
            body: vec![],
            headers,
            url: "https://www.inaturalist.org/users/api_token".try_into()?,
            method: Method::GET,
        };

        let response = oauth2::reqwest::async_http_client(request).await?;
        Ok(serde_json::from_slice(&response.body)?)
    }

    /// Configures and returns the `oauth2::BasicClient`.
    fn client(&self, redirect_url: &str) -> Result<BasicClient, Box<dyn std::error::Error>> {
        let auth_url = AuthUrl::new("https://www.inaturalist.org/oauth/authorize".to_string())?;
        let token_url = TokenUrl::new("https://www.inaturalist.org/oauth/token".to_string())?;

        Ok(BasicClient::new(
            self.client_id.clone(),
            Some(self.client_secret.clone()),
            auth_url,
            Some(token_url),
        )
        .set_redirect_uri(RedirectUrl::new(redirect_url.to_string())?))
    }
}

#[derive(Deserialize)]
struct ApiTokenResponse {
    api_token: String,
}
