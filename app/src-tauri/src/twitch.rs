use tauri::{AppHandle};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use tokio_tungstenite::tungstenite::Error as WsError;
use futures_util::{SinkExt, StreamExt};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use crate::store::TokenStore;
use tauri::Emitter;
use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::Mutex;
use lazy_static::lazy_static;

// Bandera para evitar múltiples conexiones simultáneas
static IRC_CONNECTED: AtomicBool = AtomicBool::new(false);

lazy_static! {
    // Writer global para enviar mensajes al IRC
    static ref IRC_WRITER: Mutex<Option<futures_util::stream::SplitSink<
        tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
        Message
    >>> = Mutex::new(None);

    // Handle de la tarea de lectura para poder cancelarla en logout
    static ref IRC_TASK: Mutex<Option<tokio::task::JoinHandle<()>>> = Mutex::new(None);
}


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

// --- Decodificar valores IRC (URL-encoded) ---
fn decode_irc_value(value: &str) -> String {
    value
        .replace("\\:", ";")
        .replace("\\s", " ")
        .replace("\\\\", "\\")
}

// --- Parser para extraer nick, mensaje y tags de PRIVMSG ---
fn parse_privmsg(line: &str) -> Option<ChatMessage> {
    let mut tags_map: HashMap<String, String> = HashMap::new();
    let mut remaining = line;

    if remaining.starts_with('@') {
        if let Some(tags_end) = remaining.find(' ') {
            let tags_str = &remaining[1..tags_end];
            for tag in tags_str.split(';') {
                if let Some(eq_pos) = tag.find('=') {
                    let key = tag[..eq_pos].to_string();
                    let val = decode_irc_value(&tag[eq_pos + 1..]);
                    tags_map.insert(key, val);
                }
            }
            remaining = &remaining[tags_end + 1..];
        }
    }

    if !remaining.starts_with(':') {
        return None;
    }
    remaining = &remaining[1..];

    let nick = remaining.split('!').next()?.to_string();

    if let Some(privmsg_pos) = remaining.find(" PRIVMSG ") {
        remaining = &remaining[privmsg_pos + 9..];
        if let Some(msg_start) = remaining.find(" :") {
            let message = remaining[msg_start + 2..].to_string();
            let badges = tags_map.get("badges").cloned().filter(|b| !b.is_empty());
            let color = tags_map.get("color").cloned().filter(|c| !c.is_empty());

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

    // Guardar writer globalmente
    {
        let mut writer_lock = IRC_WRITER.lock().await;
        *writer_lock = Some(write);
    }

    // Solicitar tags y membresía
    {
        let mut writer_lock = IRC_WRITER.lock().await;
        if let Some(writer) = writer_lock.as_mut() {
            writer.send(Message::Text("CAP REQ :twitch.tv/tags twitch.tv/commands twitch.tv/membership".to_string()))
                .await.map_err(|e| e.to_string())?;
            writer.send(Message::Text(format!("PASS oauth:{}", oauth_token)))
                .await.map_err(|e| e.to_string())?;
            writer.send(Message::Text(format!("NICK {}", username)))
                .await.map_err(|e| e.to_string())?;
            writer.send(Message::Text(format!("JOIN #{}", channel)))
                .await.map_err(|e| e.to_string())?;
        }
    }

    // Escuchar mensajes
    while let Some(msg_result) = read.next().await {
        let msg_result: Result<Message, WsError> = msg_result;

        match msg_result {
            Ok(Message::Text(text)) => {
                for line in text.lines() {
                    if let Some(chat_msg) = parse_privmsg(line) {
                        let json = serde_json::to_string(&chat_msg).map_err(|e| e.to_string())?;
                        app_handle.emit("twitch_message", &json).map_err(|e| e.to_string())?;
                    }
                }
            }
            Ok(Message::Ping(_)) => {
                let mut writer_lock = IRC_WRITER.lock().await;
                if let Some(writer) = writer_lock.as_mut() {
                    writer.send(Message::Text("PONG :tmi.twitch.tv".to_string()))
                        .await.map_err(|e| e.to_string())?;
                }
            }
            _ => {}
        }
    }

    Ok(())
}

// --- Comando para obtener usuario actual ---
#[tauri::command]
pub async fn get_current_user() -> Result<String, String> {
    let client = TokenStore::instance()
        .load()
        .map_err(|e| e.to_string())?
        .ok_or("No hay token guardado, inicia OAuth primero")?;

    let client_id = std::env::var("TWITCH_CLIENT_ID").map_err(|e| e.to_string())?;
    get_channel_login(&client.access_token, &client_id).await
}

// --- Comando para iniciar conexión ---
#[tauri::command]
pub async fn start_twitch_chat(app_handle: AppHandle) -> Result<(), String> {
    if IRC_CONNECTED.swap(true, Ordering::SeqCst) {
        return Ok(());
    }
    
    let client = TokenStore::instance()
        .load()
        .map_err(|e| {
            IRC_CONNECTED.store(false, Ordering::SeqCst);
            e.to_string()
        })?
        .ok_or_else(|| {
            IRC_CONNECTED.store(false, Ordering::SeqCst);
            "No hay token guardado, inicia OAuth primero".to_string()
        })?;

    let client_id = std::env::var("TWITCH_CLIENT_ID").map_err(|e| {
        IRC_CONNECTED.store(false, Ordering::SeqCst);
        e.to_string()
    })?;
    
    let username = get_channel_login(&client.access_token, &client_id).await.map_err(|e| {
        IRC_CONNECTED.store(false, Ordering::SeqCst);
        e
    })?;
    let channel = username.clone();
    let access_token = client.access_token.clone();

    // Capturar el JoinHandle en una variable 
    let handle = tokio::spawn(async move { if let Err(e) = connect_twitch_irc(&access_token, &username, &channel, app_handle).await { eprintln!("IRC connection error: {}", e); } });
    { let mut task_lock = IRC_TASK.lock().await; *task_lock = Some(handle); }
    Ok(())
}

// --- Nuevo comando para enviar mensajes ---
#[tauri::command]
pub async fn send_twitch_message(channel: String, msg: String) -> Result<(), String> {
    let mut writer_lock = IRC_WRITER.lock().await;
    if let Some(writer) = writer_lock.as_mut() {
        let line = format!("PRIVMSG #{} :{}", channel, msg);
        writer.send(Message::Text(line))
            .await
            .map_err(|e| e.to_string())?;
        Ok(())
    } else {
        Err("No IRC connection available".to_string())
    }
}

#[tauri::command]
pub async fn logout() -> Result<(), String> {
    // 1. Cargar token actual y soltar el guard antes del await
    let client_opt = {
        let store = TokenStore::instance();
        store.load()?
    };

    if let Some(client) = client_opt {
        let client_id = std::env::var("TWITCH_CLIENT_ID").map_err(|e| e.to_string())?;
        let revoke_url = "https://id.twitch.tv/oauth2/revoke";

        // 2. Revocar token en Twitch
        let http = reqwest::Client::new();
        let res = http.post(revoke_url)
            .query(&[("client_id", client_id), ("token", client.access_token.clone())])
            .send()
            .await
            .map_err(|e| e.to_string())?;

        if !res.status().is_success() {
            eprintln!("Warning: token revoke failed: {}", res.status());
        }
    }

    // 3. Borrar token local en sled
    {
        let store = TokenStore::instance();
        store.delete()?;
    }

    // 4. Resetear estado IRC
    IRC_CONNECTED.store(false, Ordering::SeqCst);
    {
        let mut writer_lock = IRC_WRITER.lock().await;
        *writer_lock = None;
    }

    // 5. Cancelar la tarea de lectura si existe
    {
        let mut task_lock = IRC_TASK.lock().await;
        if let Some(handle) = task_lock.take() {
            handle.abort(); // esto mata el bucle while del reader
        }
    }

    Ok(())
}



