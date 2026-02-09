use tauri::{AppHandle};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio_tungstenite::tungstenite::Error as WsError;
use futures_util::{SinkExt, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::store::TokenStore;
use tauri::Emitter;
use std::collections::HashMap;

#[derive(Deserialize)]
struct UserResponse {
    data: Vec<User>,
}

#[derive(Deserialize)]
struct User {
    id: String,
    login: String,
    display_name: String,
}

#[derive(Serialize, Clone)]
struct ChatMessage {
    nick: String,
    message: String,
    badges: Option<String>,
    color: Option<String>,
    raw: String,
}

// --- Obtener el login del usuario autenticado ---
async fn get_channel_login(access_token: &str, client_id: &str) -> Result<String, String> {
    let client = Client::new();
    let resp = client
        .get("https://api.twitch.tv/helix/users")
        .header("Authorization", format!("Bearer {}", access_token))
        .header("Client-Id", client_id)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let status = resp.status();
    if !status.is_success() {
        let body = resp.text().await.map_err(|e| e.to_string())?;
        return Err(format!("Helix API error {}: {}", status, body));
    }

    let res = resp.json::<UserResponse>().await.map_err(|e| e.to_string())?;

    if let Some(user) = res.data.get(0) {
        Ok(user.login.clone())
    } else {
        Err("No user found".to_string())
    }
}

// --- Parser para extraer nick, mensaje y tags de PRIVMSG ---
fn parse_privmsg(line: &str) -> Option<ChatMessage> {
    // Formato: @tags :nick!user@host PRIVMSG #channel :message
    let mut tags_map: HashMap<String, String> = HashMap::new();
    let mut remaining = line;

    // Extraer tags (si existen)
    if remaining.starts_with('@') {
        if let Some(tags_end) = remaining.find(' ') {
            let tags_str = &remaining[1..tags_end];
            for tag in tags_str.split(';') {
                if let Some(eq_pos) = tag.find('=') {
                    let key = tag[..eq_pos].to_string();
                    let val = tag[eq_pos + 1..].to_string();
                    tags_map.insert(key, val);
                }
            }
            remaining = &remaining[tags_end + 1..];
        }
    }

    // Extraer nick
    if !remaining.starts_with(':') {
        return None;
    }
    remaining = &remaining[1..];

    let nick = remaining
        .split('!')
        .next()?
        .to_string();

    // Buscar PRIVMSG
    if let Some(privmsg_pos) = remaining.find(" PRIVMSG ") {
        remaining = &remaining[privmsg_pos + 9..];
        // Saltar el canal
        if let Some(msg_start) = remaining.find(" :") {
            let message = remaining[msg_start + 2..].to_string();
            let badges = tags_map.get("badges").cloned();
            let color = tags_map.get("color").cloned();

            return Some(ChatMessage {
                nick,
                message,
                badges,
                color,
                raw: line.to_string(),
            });
        }
    }

    None
}

// --- Conexión al IRC ---
async fn connect_twitch_irc(
    oauth_token: &str,
    username: &str,
    channel: &str,
    app_handle: AppHandle,
) -> Result<(), String> {
    let url = "wss://irc-ws.chat.twitch.tv:443";
    let (ws_stream, _) = connect_async(url).await.map_err(|e| e.to_string())?;
    let (mut write, mut read) = ws_stream.split();

    // Solicitar tags y membresía
    write.send(Message::Text("CAP REQ :twitch.tv/tags twitch.tv/commands twitch.tv/membership".to_string()))
        .await
        .map_err(|e| e.to_string())?;

    // Autenticación
    write.send(Message::Text(format!("PASS oauth:{}", oauth_token)))
        .await
        .map_err(|e| e.to_string())?;
    write.send(Message::Text(format!("NICK {}", username)))
        .await
        .map_err(|e| e.to_string())?;
    write.send(Message::Text(format!("JOIN #{}", channel)))
        .await
        .map_err(|e| e.to_string())?;

    // Escuchar mensajes en tiempo real
    while let Some(msg_result) = read.next().await {
        let msg_result: Result<Message, WsError> = msg_result;

        match msg_result {
            Ok(Message::Text(text)) => {
                // Procesar cada línea (IRC puede enviar múltiples en una)
                for line in text.lines() {
                    if let Some(chat_msg) = parse_privmsg(line) {
                        let json = serde_json::to_string(&chat_msg).map_err(|e| e.to_string())?;
                        app_handle.emit("twitch_message", &json).map_err(|e| e.to_string())?;
                    }
                }
            }
            Ok(Message::Ping(_)) => {
                write.send(Message::Text("PONG :tmi.twitch.tv".to_string()))
                    .await
                    .map_err(|e| e.to_string())?;
            }
            _ => {}
        }
    }

    Ok(())
}

// --- Comando Tauri ---
#[tauri::command]
pub async fn get_current_user() -> Result<String, String> {
    // Cargar token desde el store único
    let client = TokenStore::instance()
        .load()
        .map_err(|e| e.to_string())?
        .ok_or("No hay token guardado, inicia OAuth primero")?;

    // Obtener el login del usuario con Helix
    let client_id = std::env::var("TWITCH_CLIENT_ID").map_err(|e| e.to_string())?;
    get_channel_login(&client.access_token, &client_id).await
}

#[tauri::command]
pub async fn start_twitch_chat(app_handle: AppHandle) -> Result<(), String> {
    // Cargar token desde el store único
    let client = TokenStore::instance()
        .load()
        .map_err(|e| e.to_string())?
        .ok_or("No hay token guardado, inicia OAuth primero")?;

    // Obtener el login del usuario con Helix
    let client_id = std::env::var("TWITCH_CLIENT_ID").map_err(|e| e.to_string())?;
    let username = get_channel_login(&client.access_token, &client_id).await?;
    let channel = username.clone();

    connect_twitch_irc(&client.access_token, &username, &channel, app_handle).await
}
