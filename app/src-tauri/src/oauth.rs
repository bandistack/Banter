use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use reqwest::Response;
use crate::api::api_call;
use crate::store::RefreshTokenStore;

#[derive(Deserialize, Serialize, Debug, Clone)] 
pub struct OAuthClient { 
    pub access_token: String, 
    pub refresh_token: Option<String>, 
    pub expires_in: u64, 
    pub scope: Vec<String>, 
    pub token_type: String, 
}

impl OAuthClient {
    pub async fn request_token(code: String, grant_type: &str) -> Result<Self, String> {
        let mut params: HashMap<String, String> = HashMap::from([
            ("client_id".to_string(), std::env::var("TWITCH_CLIENT_ID").expect("TWITCH_CLIENT_ID not set")),
            ("client_secret".to_string(), std::env::var("TWITCH_CLIENT_SECRET").expect("TWITCH_CLIENT_SECRET not set")),
            ("grant_type".to_string(), grant_type.to_string()),
        ]);
        match grant_type {
            "authorization_code" => {
                params.insert("code".to_string(), code);
                params.insert("redirect_uri".to_string(), std::env::var("TWITCH_REDIRECT_URI").expect("TWITCH_REDIRECT_URI not set"));
            } 
            "refresh_token" => { 
                params.insert("refresh_token".to_string(), code); 
            } 
            _ => return Err("Unsupported grant_type".to_string()), 
        }
        let res: Response = api_call("https://id.twitch.tv/oauth2/token", &params)
            .await
            .map_err(|e| e.to_string())?;
        if !res.status().is_success() { 
            return Err(format!("HTTP error: {}", res.status())); 
        }
        let token_response = res.json::<Self>()
            .await
            .map_err(|e| e.to_string())?;
        if let Some(refresh) = &token_response.refresh_token { 
            RefreshTokenStore::new()
                .map_err(|e| e.to_string())?
                .save(refresh)
                .map_err(|e| e.to_string())?;
        }
        Ok(token_response)
    }
    pub async fn exchange_token(code: String) -> Result<Self, String> { 
        Self::request_token(code, "authorization_code").await 
    } 
    pub async fn refresh_token(refresh: String) -> Result<Self, String> { 
        Self::request_token(refresh, "refresh_token").await 
    }
}

