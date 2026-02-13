use crate::models::{OAuthTokens, OAuthTokensBuilder};
use crate::store::TokenStore;

#[tauri::command]
pub async fn chtk(actk: String, idtk: Option<String>) -> Result<(), String> {
    if actk.trim().is_empty() {
        return Err("access_token vacio".to_string());
    }
    let tokens = OAuthTokens::builder(actk).id_token(idtk).build();
    TokenStore::instance().save(&tokens)?;
    Ok(())
}