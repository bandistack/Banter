use sled::Db;
use serde_json;
use once_cell::sync::Lazy;
use std::sync::Mutex;
use std::path::PathBuf;
use crate::models::OAuthTokens;

static STORE: Lazy<Mutex<TokenStore>> = Lazy::new(|| {
    let store = TokenStore::new().expect("Error inicializando TokenStore");
    Mutex::new(store)
});

pub struct TokenStore {
    db: Db,
}

impl TokenStore {
    pub fn new() -> sled::Result<Self> {
        let mut base_path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
        base_path.push("Bs");
        base_path.push("ud");
        Ok(TokenStore { db: sled::open(base_path)? })
    }

    pub fn instance() -> std::sync::MutexGuard<'static, TokenStore> {
        STORE.lock().unwrap()
    }

    /// Update/Save tokens
    pub fn save(&self, tokens: &OAuthTokens) -> Result<(), String> {
        let data = serde_json::to_vec(tokens).map_err(|e| e.to_string())?;
        self.db.insert("oauth_tokens", data).map_err(|e| e.to_string())?;
        self.db.flush().map_err(|e| e.to_string())?;
        Ok(())
    }

    /// Load tokens
    pub fn load(&self) -> Result<Option<OAuthTokens>, String> {
        if let Some(ivec) = self.db.get("oauth_tokens").map_err(|e| e.to_string())? {
            let tokens: OAuthTokens = serde_json::from_slice(&ivec).map_err(|e| e.to_string())?;
            Ok(Some(tokens))
        } else {
            Ok(None)
        }
    }

    pub fn delete(&self) -> Result<(), String> {
        self.db.remove("oauth_tokens").map_err(|e| e.to_string())?;
        self.db.flush().map_err(|e| e.to_string())?;
        Ok(())
    }
}
#[tauri::command]
pub async fn logout() -> Result<(), String> {
    TokenStore::instance().delete()
}