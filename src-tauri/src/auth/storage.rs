use super::AuthSession;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

/// Get the auth storage file path
/// Uses Application Support directory for persistent storage
fn get_auth_file_path() -> PathBuf {
    let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
    let app_support = PathBuf::from(&home).join("Library/Application Support/com.promptlight");
    fs::create_dir_all(&app_support).ok();
    app_support.join("auth_session.json")
}

/// Auth state returned to the frontend
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthState {
    pub is_signed_in: bool,
    pub user: Option<super::User>,
}

/// Save auth session to file storage
pub fn save_auth_session(session: &AuthSession) -> Result<(), String> {
    let path = get_auth_file_path();

    let json = serde_json::to_string_pretty(session)
        .map_err(|e| format!("Failed to serialize session: {}", e))?;

    write_private_file(&path, &json)
        .map_err(|e| format!("Failed to save auth session: {}", e))?;

    Ok(())
}

/// Write a file readable only by the current user (0600 on Unix).
/// The session contains a long-lived refresh token, so it must not be
/// world-readable. Also tightens permissions on a pre-existing file.
fn write_private_file(path: &PathBuf, contents: &str) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        use std::io::Write;
        use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};

        let mut file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(path)?;
        file.set_permissions(fs::Permissions::from_mode(0o600))?;
        file.write_all(contents.as_bytes())
    }

    #[cfg(not(unix))]
    {
        fs::write(path, contents)
    }
}

/// Load auth session from file storage
pub fn load_auth_session() -> Option<AuthSession> {
    let path = get_auth_file_path();
    let json = fs::read_to_string(&path).ok()?;
    serde_json::from_str(&json).ok()
}

/// Clear auth session from file storage
pub fn clear_auth() -> Result<(), String> {
    let path = get_auth_file_path();
    let _ = fs::remove_file(&path);
    Ok(())
}

/// Get current auth state (for frontend)
pub fn get_auth_state() -> AuthState {
    match load_auth_session() {
        Some(session) => {
            // Check if token is expired
            let now = chrono::Utc::now().timestamp();
            if session.tokens.expires_at <= now {
                AuthState {
                    is_signed_in: false,
                    user: None,
                }
            } else {
                AuthState {
                    is_signed_in: true,
                    user: Some(session.user),
                }
            }
        }
        None => AuthState {
            is_signed_in: false,
            user: None,
        },
    }
}

#[cfg(all(test, unix))]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;

    #[test]
    fn test_write_private_file_is_owner_only() {
        let path = std::env::temp_dir().join(format!("promptlight-auth-test-{}", uuid::Uuid::new_v4()));
        // Pre-existing world-readable file must be tightened
        fs::write(&path, "old").unwrap();
        fs::set_permissions(&path, fs::Permissions::from_mode(0o644)).unwrap();

        write_private_file(&path, "secret").unwrap();

        let mode = fs::metadata(&path).unwrap().permissions().mode() & 0o777;
        assert_eq!(mode, 0o600);
        assert_eq!(fs::read_to_string(&path).unwrap(), "secret");
        fs::remove_file(&path).ok();
    }
}
