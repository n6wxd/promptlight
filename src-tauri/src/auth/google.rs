use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine};
use rand::Rng;
use sha2::{Digest, Sha256};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::sync::OnceLock;
use tauri::AppHandle;
use tauri_plugin_opener::OpenerExt;
use url::Url;

/// Google OAuth configuration
const GOOGLE_AUTH_URL: &str = "https://accounts.google.com/o/oauth2/v2/auth";
const GOOGLE_TOKEN_URL: &str = "https://oauth2.googleapis.com/token";

/// Get the Google OAuth client ID
/// Uses compile-time env for release builds, falls back to runtime for dev
fn get_google_client_id() -> &'static str {
    static CLIENT_ID: OnceLock<String> = OnceLock::new();
    CLIENT_ID.get_or_init(|| {
        // First try compile-time (for release builds)
        if let Some(id) = option_env!("GOOGLE_CLIENT_ID") {
            return id.to_string();
        }
        // Fall back to runtime (for dev builds with .env.local)
        std::env::var("GOOGLE_CLIENT_ID")
            .expect("GOOGLE_CLIENT_ID must be set (compile-time or in .env.local)")
    })
}

/// Get the Google OAuth client secret
/// Uses compile-time env for release builds, falls back to runtime for dev
fn get_google_client_secret() -> &'static str {
    static CLIENT_SECRET: OnceLock<String> = OnceLock::new();
    CLIENT_SECRET.get_or_init(|| {
        // First try compile-time (for release builds)
        if let Some(secret) = option_env!("GOOGLE_CLIENT_SECRET") {
            return secret.to_string();
        }
        // Fall back to runtime (for dev builds with .env.local)
        std::env::var("GOOGLE_CLIENT_SECRET")
            .expect("GOOGLE_CLIENT_SECRET must be set (compile-time or in .env.local)")
    })
}

/// Tokens returned from Google OAuth
#[derive(Debug, Clone)]
pub struct GoogleTokens {
    pub id_token: String,
    pub access_token: String,
    pub refresh_token: Option<String>,
}

/// Generate a random code verifier for PKCE
fn generate_code_verifier() -> String {
    let mut rng = rand::thread_rng();
    let bytes: Vec<u8> = (0..32).map(|_| rng.gen()).collect();
    URL_SAFE_NO_PAD.encode(&bytes)
}

/// Generate code challenge from verifier (SHA256)
fn generate_code_challenge(verifier: &str) -> String {
    let mut hasher = Sha256::new();
    hasher.update(verifier.as_bytes());
    let hash = hasher.finalize();
    URL_SAFE_NO_PAD.encode(hash)
}

/// Open a URL using NSWorkspace (native macOS API)
/// Returns true if the URL was successfully opened
#[cfg(target_os = "macos")]
fn open_url_with_nsworkspace(url_string: &str) -> bool {
    use cocoa::base::{id, nil, BOOL, YES};
    use cocoa::foundation::NSString;
    use objc::{class, msg_send, sel, sel_impl};

    unsafe {
        // Create NSURL from string
        let ns_url_string: id = NSString::alloc(nil).init_str(url_string);
        let ns_url: id = msg_send![class!(NSURL), URLWithString: ns_url_string];

        if ns_url == nil {
            println!("[auth] ERROR: Failed to create NSURL from string");
            return false;
        }

        // Get shared NSWorkspace
        let workspace: id = msg_send![class!(NSWorkspace), sharedWorkspace];
        if workspace == nil {
            println!("[auth] ERROR: Failed to get NSWorkspace");
            return false;
        }

        // Open URL with default browser
        let result: BOOL = msg_send![workspace, openURL: ns_url];

        result == YES
    }
}

/// Start local server, open browser for OAuth, and wait for callback
pub async fn start_google_sign_in(app: &AppHandle) -> Result<GoogleTokens, String> {
    println!("[auth] ====== OAUTH FLOW START ======");
    println!("[auth] Step 1: Generating PKCE values...");

    // Generate PKCE values
    let code_verifier = generate_code_verifier();
    let code_challenge = generate_code_challenge(&code_verifier);
    // CSRF protection: callback must echo this back
    let state = generate_code_verifier();
    println!("[auth] Step 1: COMPLETE - PKCE values generated");

    // Find an available port
    println!("[auth] Step 2: Binding TCP listener...");
    let listener = TcpListener::bind("127.0.0.1:0")
        .map_err(|e| {
            println!("[auth] Step 2: FAILED - {}", e);
            format!("Failed to bind local server: {}", e)
        })?;
    let port = listener
        .local_addr()
        .map_err(|e| {
            println!("[auth] Step 2: FAILED to get port - {}", e);
            format!("Failed to get local address: {}", e)
        })?
        .port();
    println!("[auth] Step 2: COMPLETE - Bound to port {}", port);

    let redirect_uri = format!("http://127.0.0.1:{}", port);

    // Build the authorization URL
    println!("[auth] Step 3: Building auth URL...");
    let auth_url = build_auth_url(&redirect_uri, &code_challenge, &state)?;
    println!("[auth] Step 3: COMPLETE - Auth URL built");
    println!("[auth] Step 4: Opening browser...");

    // Open the browser using NSWorkspace (native macOS API)
    // This is the most reliable method on macOS 15 Sequoia
    #[cfg(target_os = "macos")]
    {
        println!("[auth] === BROWSER OPEN ATTEMPT ===");
        println!("[auth] Full auth URL: {}", &auth_url);

        // Try NSWorkspace first (native macOS API)
        println!("[auth] Method 1: Trying NSWorkspace.openURL...");
        let ns_result = open_url_with_nsworkspace(&auth_url);
        println!("[auth] NSWorkspace returned: {}", ns_result);

        if !ns_result {
            println!("[auth] NSWorkspace failed, trying /usr/bin/open...");

            // Fallback 1: Try /usr/bin/open command
            match std::process::Command::new("/usr/bin/open")
                .arg(&auth_url)
                .output()
            {
                Ok(output) => {
                    println!("[auth] /usr/bin/open exit code: {:?}", output.status.code());
                    if !output.status.success() {
                        let stderr = String::from_utf8_lossy(&output.stderr);
                        println!("[auth] /usr/bin/open stderr: {}", stderr);

                        // Fallback 2: Try Tauri opener
                        println!("[auth] Trying Tauri opener...");
                        match app.opener().open_url(&auth_url, None::<&str>) {
                            Ok(_) => println!("[auth] Tauri opener returned Ok"),
                            Err(e) => {
                                println!("[auth] ERROR: All methods failed. Tauri error: {}", e);
                                return Err(format!("Could not open browser. Check Console.app for [auth] logs."));
                            }
                        }
                    }
                }
                Err(e) => {
                    println!("[auth] /usr/bin/open error: {}", e);
                    // Try Tauri opener
                    match app.opener().open_url(&auth_url, None::<&str>) {
                        Ok(_) => println!("[auth] Tauri opener returned Ok"),
                        Err(e2) => {
                            println!("[auth] ERROR: All methods failed: {}, {}", e, e2);
                            return Err(format!("Could not open browser. Check Console.app for [auth] logs."));
                        }
                    }
                }
            }
        }

        println!("[auth] Step 4: COMPLETE - Browser open returned success");
    }

    #[cfg(not(target_os = "macos"))]
    {
        match app.opener().open_url(&auth_url, None::<&str>) {
            Ok(_) => println!("[auth] Browser open command succeeded"),
            Err(e) => {
                println!("[auth] ERROR: Failed to open browser: {}", e);
                return Err(format!("Failed to open browser: {}", e));
            }
        }
    }

    // Wait for the OAuth callback in a blocking task to not block the async runtime
    // This is critical: TcpListener::accept() is blocking and would freeze the app
    let code = tokio::task::spawn_blocking(move || wait_for_callback(listener, &state))
        .await
        .map_err(|e| format!("Task join error: {}", e))?
        .map_err(|e| format!("OAuth callback error: {}", e))?;

    // Exchange the authorization code for tokens
    let tokens = exchange_code_for_tokens(&code, &code_verifier, &redirect_uri).await?;

    Ok(tokens)
}

/// Build the Google OAuth authorization URL
fn build_auth_url(redirect_uri: &str, code_challenge: &str, state: &str) -> Result<String, String> {
    let mut url = Url::parse(GOOGLE_AUTH_URL).map_err(|e| e.to_string())?;

    url.query_pairs_mut()
        .append_pair("client_id", get_google_client_id())
        .append_pair("redirect_uri", redirect_uri)
        .append_pair("response_type", "code")
        .append_pair("scope", "openid email profile")
        .append_pair("code_challenge", code_challenge)
        .append_pair("code_challenge_method", "S256")
        .append_pair("state", state)
        .append_pair("access_type", "offline")
        .append_pair("prompt", "consent");

    Ok(url.to_string())
}

/// Outcome of parsing one request to the local callback server
#[derive(Debug, PartialEq)]
enum CallbackResult {
    /// Valid callback with the authorization code
    Code(String),
    /// Valid callback (matching state) reporting an OAuth error
    Error(String),
    /// Not our callback (missing/mismatched state, favicon, etc.) - ignore it
    Ignore,
}

/// Wait for the OAuth callback on the local server.
///
/// Keeps accepting connections until one carries the expected `state`, so a
/// stray or hostile local request can neither inject a code nor abort sign-in.
fn wait_for_callback(listener: TcpListener, expected_state: &str) -> Result<String, String> {
    loop {
        let (mut stream, _) = listener
            .accept()
            .map_err(|e| format!("Failed to accept connection: {}", e))?;

        // Read the HTTP request line
        let mut request_line = String::new();
        if BufReader::new(&stream).read_line(&mut request_line).is_err() {
            continue;
        }

        let result = parse_callback(&request_line, expected_state);
        if result == CallbackResult::Ignore {
            let _ = stream.write_all(
                b"HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\nConnection: close\r\n\r\n",
            );
            continue;
        }

        // Send a success response
        let response = "HTTP/1.1 200 OK\r\n\
            Content-Type: text/html\r\n\
            Connection: close\r\n\r\n\
            <html><body>\
            <h1>Sign-in successful!</h1>\
            <p>You can close this window and return to PromptLight.</p>\
            <script>window.close();</script>\
            </body></html>";

        stream
            .write_all(response.as_bytes())
            .map_err(|e| format!("Failed to send response: {}", e))?;

        return match result {
            CallbackResult::Code(code) => Ok(code),
            CallbackResult::Error(error) => Err(format!("OAuth error: {}", error)),
            CallbackResult::Ignore => unreachable!(),
        };
    }
}

/// Parse an HTTP request line from the OAuth redirect.
/// Request line looks like: "GET /?state=...&code=abc123&scope=... HTTP/1.1"
fn parse_callback(request_line: &str, expected_state: &str) -> CallbackResult {
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    if parts.len() < 2 {
        return CallbackResult::Ignore;
    }

    let url = match Url::parse(&format!("http://localhost{}", parts[1])) {
        Ok(url) => url,
        Err(_) => return CallbackResult::Ignore,
    };
    let param = |name: &str| {
        url.query_pairs()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.to_string())
    };

    if param("state").as_deref() != Some(expected_state) {
        return CallbackResult::Ignore;
    }

    if let Some(error) = param("error") {
        return CallbackResult::Error(error);
    }

    match param("code") {
        Some(code) => CallbackResult::Code(code),
        None => CallbackResult::Ignore,
    }
}

/// Exchange the authorization code for tokens
async fn exchange_code_for_tokens(
    code: &str,
    code_verifier: &str,
    redirect_uri: &str,
) -> Result<GoogleTokens, String> {
    let client = reqwest::Client::new();

    let params = [
        ("client_id", get_google_client_id()),
        ("client_secret", get_google_client_secret()),
        ("code", code),
        ("code_verifier", code_verifier),
        ("grant_type", "authorization_code"),
        ("redirect_uri", redirect_uri),
    ];

    let response = client
        .post(GOOGLE_TOKEN_URL)
        .form(&params)
        .send()
        .await
        .map_err(|e| format!("Token request failed: {}", e))?;

    if !response.status().is_success() {
        let error_text = response.text().await.unwrap_or_default();
        return Err(format!("Token exchange failed: {}", error_text));
    }

    let token_response: serde_json::Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse token response: {}", e))?;

    let id_token = token_response["id_token"]
        .as_str()
        .ok_or("Missing id_token")?
        .to_string();

    let access_token = token_response["access_token"]
        .as_str()
        .ok_or("Missing access_token")?
        .to_string();

    let refresh_token = token_response["refresh_token"].as_str().map(|s| s.to_string());

    Ok(GoogleTokens {
        id_token,
        access_token,
        refresh_token,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_callback_requires_matching_state() {
        let line = |q: &str| format!("GET /?{} HTTP/1.1", q);
        assert_eq!(
            parse_callback(&line("state=abc&code=xyz"), "abc"),
            CallbackResult::Code("xyz".to_string())
        );
        assert_eq!(parse_callback(&line("state=evil&code=xyz"), "abc"), CallbackResult::Ignore);
        assert_eq!(parse_callback(&line("code=xyz"), "abc"), CallbackResult::Ignore);
        assert_eq!(parse_callback(&line("error=access_denied"), "abc"), CallbackResult::Ignore);
        assert_eq!(
            parse_callback(&line("state=abc&error=access_denied"), "abc"),
            CallbackResult::Error("access_denied".to_string())
        );
        assert_eq!(parse_callback("GET /favicon.ico HTTP/1.1", "abc"), CallbackResult::Ignore);
        assert_eq!(parse_callback("garbage", "abc"), CallbackResult::Ignore);
    }

    #[test]
    fn test_auth_url_includes_state() {
        // Client ID lookup needs an env value when not set at compile time
        if option_env!("GOOGLE_CLIENT_ID").is_none() {
            std::env::set_var("GOOGLE_CLIENT_ID", "test-client");
        }
        let url = build_auth_url("http://127.0.0.1:1", "challenge", "st4te").unwrap();
        assert!(url.contains("state=st4te"));
    }
}
