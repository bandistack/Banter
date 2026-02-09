use sled::Db;
use serde_json;
use crate::oauth::OAuthClient;
use once_cell::sync::Lazy;
use std::sync::Mutex; 
use std::path::PathBuf;

// Instancia única para toda la app
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
        base_path.push("banter_app"); 
        base_path.push("tokens"); 
        return Ok(TokenStore { db: sled::open(base_path)? });
    }
    pub fn instance() -> std::sync::MutexGuard<'static, TokenStore> {
        STORE.lock().unwrap()
    }
    pub fn save(&self, client: &OAuthClient) -> Result<(), String> {
        let data = serde_json::to_vec(client).map_err(|e| e.to_string())?;
        self.db.insert("oauth_client", data).map_err(|e| e.to_string())?;
        self.db.flush().map_err(|e| e.to_string())?;
        Ok(())
    }

    pub fn load(&self) -> Result<Option<OAuthClient>, String> {
        if let Some(ivec) = self.db.get("oauth_client").map_err(|e| e.to_string())? {
            let client: OAuthClient = serde_json::from_slice(&ivec).map_err(|e| e.to_string())?;
            Ok(Some(client))
        } else {
            Ok(None)
        }
    }

    pub fn delete(&self) -> Result<(), String> {
        self.db.remove("oauth_client").map_err(|e| e.to_string())?;
        self.db.flush().map_err(|e| e.to_string())?;
        Ok(())
    }
}

