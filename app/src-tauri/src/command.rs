use crate::oauth::OAuthClient;
use crate::store::RefreshTokenStore;

#[tauri::command]
pub async fn exchange_token(code: String) -> Result<OAuthClient, String> {
    OAuthClient::exchange_token(code).await
}

#[tauri::command]
pub async fn refresh_token() -> Result<OAuthClient, String> {
    let store = RefreshTokenStore::new().map_err(|e| e.to_string())?;
    if let Some(refresh) = store.load().map_err(|e| e.to_string())? {
        OAuthClient::refresh_token(refresh).await
    } else {
        Err("No refresh token stored".to_string())
    }
}
