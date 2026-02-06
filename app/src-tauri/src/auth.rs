use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::Response;
use crate::api::post_form;

#[derive(Deserialize, Serialize, Debug)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_in: u64,
    pub scope: Vec<String>,
    pub token_type: String,
}
fn get_env_var(key: &str) -> Result<String, String> {
    std::env::var(key).map_err(|_| format!("Not defined env variable {}", key))
}
#[tauri::command]
pub async fn exchange_token(code: String) -> Result<TokenResponse, String> {  
    let client_id = get_env_var("TWITCH_CLIENT_ID")?;
    let client_secret = get_env_var("TWITCH_CLIENT_SECRET")?;
    let redirect_uri = get_env_var("TWITCH_REDIRECT_URI")?;
    let params: HashMap<String, String> = HashMap::from([
        ("client_id".to_string(), client_id),
        ("client_secret".to_string(), client_secret),
        ("code".to_string(), code),
        ("grant_type".to_string(), "authorization_code".to_string()),
        ("redirect_uri".to_string(), redirect_uri),
    ]);
    let res: Response = post_form("https://id.twitch.tv/oauth2/token", &params)
        .await
        .map_err(|e| e.to_string())?;
    if res.status().is_success() {
        res.json::<TokenResponse>().await.map_err(|e| e.to_string())
    } else {
        Err(format!("HTTP error: {}", res.status()))
    }
}
