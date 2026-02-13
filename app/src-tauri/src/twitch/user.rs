use crate::store::TokenStore;

/// Lee el campo `preferred_username` del id_token JWT guardado en sled.
pub fn current_user() -> Result<String, String> {
    let tokens = TokenStore::instance().load()?.ok_or("Sin sesión")?;
    let id_token = tokens.id_token.ok_or("Sin id_token")?;

    let payload = id_token.split('.').nth(1).ok_or("id_token malformado")?;
    let decoded = base64url_decode(payload)?;
    let json: serde_json::Value = serde_json::from_slice(&decoded).map_err(|e| e.to_string())?;

    json["preferred_username"]
        .as_str()
        .map(|s| s.to_string())
        .ok_or("preferred_username no encontrado".into())
}

fn base64url_decode(s: &str) -> Result<Vec<u8>, String> {
    let padded = match s.len() % 4 {
        2 => format!("{s}=="),
        3 => format!("{s}="),
        _ => s.to_string(),
    };
    base64::decode(padded.replace('-', "+").replace('_', "/")).map_err(|e| e.to_string())
}