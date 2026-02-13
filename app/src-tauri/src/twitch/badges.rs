use once_cell::sync::Lazy;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::sync::RwLock;

// ── Estructuras de la API de Twitch ─────────────────────────────────────────

#[derive(Debug, Deserialize)]
struct BadgeResponse {
    data: Vec<BadgeSet>,
}

#[derive(Debug, Deserialize)]
struct BadgeSet {
    set_id: String,
    versions: Vec<BadgeVersion>,
}

#[derive(Debug, Deserialize)]
struct BadgeVersion {
    id: String,
    image_url_1x: String,
    image_url_2x: String,
}

// ── Caché en memoria ─────────────────────────────────────────────────────────
// Clave: "set_id/version_id"  →  BadgeUrls
#[derive(Debug, Clone, Serialize)]
pub struct BadgeUrls {
    pub url_1x: String,
    pub url_2x: String,
}

// caché global: set_id/version → urls
static BADGE_CACHE: Lazy<RwLock<HashMap<String, BadgeUrls>>> =
    Lazy::new(|| RwLock::new(HashMap::new()));

// ── Carga badges globales + de canal ────────────────────────────────────────

pub async fn load_badges(client_id: &str, token: &str, broadcaster_id: &str) -> Result<(), String> {
    let client = Client::new();
    let mut cache = BADGE_CACHE.write().await;

    // 1. Badges globales
    let global = client
        .get("https://api.twitch.tv/helix/chat/badges/global")
        .header("Client-Id", client_id)
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<BadgeResponse>()
        .await
        .map_err(|e| e.to_string())?;

    for set in global.data {
        for v in set.versions {
            cache.insert(
                format!("{}/{}", set.set_id, v.id),
                BadgeUrls { url_1x: v.image_url_1x, url_2x: v.image_url_2x },
            );
        }
    }

    // 2. Badges del canal (sobreescriben globales si hay conflicto)
    let channel = client
        .get(format!(
            "https://api.twitch.tv/helix/chat/badges?broadcaster_id={broadcaster_id}"
        ))
        .header("Client-Id", client_id)
        .header("Authorization", format!("Bearer {token}"))
        .send()
        .await
        .map_err(|e| e.to_string())?
        .json::<BadgeResponse>()
        .await
        .map_err(|e| e.to_string())?;

    for set in channel.data {
        for v in set.versions {
            cache.insert(
                format!("{}/{}", set.set_id, v.id),
                BadgeUrls { url_1x: v.image_url_1x, url_2x: v.image_url_2x },
            );
        }
    }

    Ok(())
}

// ── Comando Tauri ────────────────────────────────────────────────────────────

/// Devuelve las URLs de una lista de badge ids (ej. ["moderator", "subscriber"])
/// El frontend pasa los badges que vienen en cada ChatMessage
#[tauri::command]
pub async fn resolve_badges(badge_ids: Vec<String>) -> HashMap<String, BadgeUrls> {
    let cache = BADGE_CACHE.read().await;
    badge_ids
        .into_iter()
        .filter_map(|id| {
            // badge_ids viene como "moderator", "subscriber/1", etc.
            // intentamos con version "1" si no tiene versión
            let key = if id.contains('/') { id.clone() } else { format!("{id}/1") };
            cache.get(&key).map(|urls| (id, urls.clone()))
        })
        .collect()
}

/// Carga badges leyendo client_id y token directamente del store
pub async fn load_badges_from_store(broadcaster_id: &str) -> Result<(), String> {
    let tokens = crate::store::TokenStore::instance()
        .load()?
        .ok_or("Sin sesión")?;
    let token = tokens.access_token.clone();
    let client_id = tokens.client_id.clone().ok_or("Sin client_id en store")?;
    load_badges(&client_id, &token, broadcaster_id).await
}

/// Recarga el caché de badges (útil si cambia el canal)
#[tauri::command]
pub async fn reload_badges(broadcaster_id: String) -> Result<(), String> {
    load_badges_from_store(&broadcaster_id).await
}