use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::{Client, Response};
use crate::store::TokenStore;

#[derive(Deserialize, Serialize, Debug, Clone)] 
pub struct OAuthClient { 
    pub access_token: String, 
    pub refresh_token: Option<String>, 
    pub expires_in: u64, 
    pub scope: Vec<String>, 
    pub token_type: String, 
}
#[tauri::command]
pub async fn exchange_token(code: String) -> Result<OAuthClient, String> {
    let params: HashMap<String, String> = HashMap::from([
        ("client_id".to_string(), std::env::var("TWITCH_CLIENT_ID").expect("TWITCH_CLIENT_ID not set")),
        ("client_secret".to_string(), std::env::var("TWITCH_CLIENT_SECRET").expect("TWITCH_CLIENT_SECRET not set")),
        ("grant_type".to_string(), "authorization_code".to_string()),
        ("code".to_string(), code),
        ("redirect_uri".to_string(), std::env::var("TWITCH_REDIRECT_URI").expect("TWITCH_REDIRECT_URI not set")),
    ]);
    let res: Response = api_call("https://id.twitch.tv/oauth2/token", &params)
        .await
        .map_err(|e| e.to_string())?;
    if !res.status().is_success() {
        return Err(format!("HTTP error: {}", res.status()));
    }

    let token_response = res.json::<OAuthClient>().await.map_err(|e| e.to_string())?;

    TokenStore::instance().save(&token_response).map_err(|e| e.to_string())?;

    Ok(token_response)
}
#[tauri::command]
pub async fn refresh_token() -> Result<OAuthClient, String> {
    // Cargar el objeto completo desde el store
    let client = TokenStore::instance()
        .load()
        .map_err(|e| e.to_string())?
        .ok_or("No hay token guardado, inicia OAuth primero")?;

    // Verificar que exista refresh_token
    let refresh = client
        .refresh_token
        .ok_or("No hay refresh_token disponible")?;

    // Construir parámetros para la API
    let params: HashMap<String, String> = HashMap::from([
        ("client_id".to_string(), std::env::var("TWITCH_CLIENT_ID").expect("TWITCH_CLIENT_ID not set")),
        ("client_secret".to_string(), std::env::var("TWITCH_CLIENT_SECRET").expect("TWITCH_CLIENT_SECRET not set")),
        ("grant_type".to_string(), "refresh_token".to_string()),
        ("refresh_token".to_string(), refresh),
    ]);

    // Llamada a la API
    let res: Response = api_call("https://id.twitch.tv/oauth2/token", &params)
        .await
        .map_err(|e| e.to_string())?;
    if !res.status().is_success() {
        return Err(format!("HTTP error: {}", res.status()));
    }

    let token_response = res.json::<OAuthClient>().await.map_err(|e| e.to_string())?;

    // Guardar el objeto actualizado
    TokenStore::instance().save(&token_response).map_err(|e| e.to_string())?;

    Ok(token_response)
}
pub async fn api_call(url: &str, params: &HashMap<String, String>) -> Result<Response, String> {
    Ok(
        Client::new()
            .post(url)
            .form(params)
            .send()
            .await
            .map_err(|e| e.to_string())?
    )
}
