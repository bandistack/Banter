use crate::store::TokenStore;
use crate::twitch::parser::{parse_privmsg, parse_tags};
use futures_util::{SinkExt, StreamExt};
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::sync::{atomic::{AtomicBool, Ordering}, Arc};
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use tokio::sync::{mpsc, Mutex};
use tokio_tungstenite::{connect_async, tungstenite::Message};

const WS_URL: &str = "wss://irc-ws.chat.twitch.tv:443";

pub(super) struct Actor {
    pub tx: Option<mpsc::Sender<String>>,
    pub channel: Option<String>,
}

pub(super) static ACTOR: Lazy<Mutex<Actor>> =
    Lazy::new(|| Mutex::new(Actor { tx: None, channel: None }));

pub(super) static RUNNING: Lazy<Arc<AtomicBool>> =
    Lazy::new(|| Arc::new(AtomicBool::new(false)));

pub async fn connect(channel: String, app: AppHandle) -> Result<(), String> {
    if RUNNING.load(Ordering::SeqCst) {
        return Err("Ya conectado".into());
    }

    let token = TokenStore::instance().load()?.ok_or("Sin sesión")?.access_token;
    let username = crate::twitch::user::current_user()?;

    let ch = if channel.starts_with('#') {
        channel.to_lowercase()
    } else {
        format!("#{}", channel.to_lowercase())
    };

    RUNNING.store(true, Ordering::SeqCst);
    ACTOR.lock().await.channel = Some(ch.clone());
    tokio::spawn(run_loop(token, username, ch, app));
    Ok(())
}

pub fn disconnect() {
    RUNNING.store(false, Ordering::SeqCst);
    if let Ok(guard) = ACTOR.try_lock() {
        if let (Some(tx), Some(ch)) = (guard.tx.clone(), guard.channel.clone()) {
            let _ = tx.try_send(format!("PART {ch}"));
        }
    }
}

pub async fn send(text: &str) -> Result<(), String> {
    let tx;
    let ch;
    {
        let guard = ACTOR.lock().await;
        tx = guard.tx.clone().ok_or("Sin conexión")?;
        ch = guard.channel.clone().ok_or("Sin canal")?;
    }
    tx.send(format!("PRIVMSG {ch} :{text}")).await.map_err(|e| e.to_string())
}

// ── Loop interno ─────────────────────────────────────────────────────────────

async fn run_loop(token: String, username: String, channel: String, app: AppHandle) {
    while RUNNING.load(Ordering::SeqCst) {
        let _ = app.emit("twitch:status", "connecting");
        match session(&token, &username, &channel, &app).await {
            Ok(_) => break,
            Err(e) => {
                eprintln!("[twitch] {e}");
                let _ = app.emit("twitch:status", "reconnecting");
                tokio::time::sleep(Duration::from_secs(3)).await;
            }
        }
    }
    ACTOR.lock().await.tx = None;
    let _ = app.emit("twitch:status", "disconnected");
}

async fn session(token: &str, username: &str, channel: &str, app: &AppHandle) -> Result<(), String> {
    let (ws, _) = connect_async(WS_URL).await.map_err(|e| e.to_string())?;
    let (mut write, mut read) = ws.split();
    let (tx, mut rx) = mpsc::channel::<String>(64);

    ACTOR.lock().await.tx = Some(tx.clone());

    for msg in [
        "CAP REQ :twitch.tv/tags twitch.tv/commands".to_string(),
        format!("PASS oauth:{token}"),
        format!("NICK {username}"),   // ← usuario real, no justinfan1
        format!("JOIN {channel}"),
    ] {
        write.send(Message::Text(msg)).await.map_err(|e| e.to_string())?;
    }
    let _ = app.emit("twitch:status", "connected");

    let stop = RUNNING.clone();
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if !stop.load(Ordering::SeqCst) || write.send(Message::Text(msg)).await.is_err() {
                break;
            }
        }
        let _ = write.send(Message::Close(None)).await;
    });

    loop {
        if !RUNNING.load(Ordering::SeqCst) { return Ok(()); }
        match tokio::time::timeout(Duration::from_secs(360), read.next()).await {
            Err(_)           => return Err("Timeout".into()),
            Ok(None)         => return Err("WS cerrado".into()),
            Ok(Some(Err(e))) => return Err(e.to_string()),
            Ok(Some(Ok(Message::Text(raw)))) => dispatch(&raw, &tx, app).await,
            _ => {}
        }
    }
}

async fn dispatch(raw: &str, tx: &mpsc::Sender<String>, app: &AppHandle) {
    for line in raw.lines().map(str::trim).filter(|l| !l.is_empty()) {
        if let Some(srv) = line.strip_prefix("PING ") {
            let _ = tx.try_send(format!("PONG {srv}"));
            continue;
        }

        let (tags, rest) = match line.strip_prefix('@') {
            Some(t) => {
                let i = t.find(' ').unwrap_or(t.len());
                (parse_tags(&t[..i]), t[i..].trim())
            }
            None => (HashMap::new(), line),
        };
        let (prefix, rest) = match rest.strip_prefix(':') {
            Some(r) => {
                let i = r.find(' ').unwrap_or(r.len());
                (Some(&r[..i]), r[i..].trim())
            }
            None => (None, rest),
        };
        let (cmd, params) = rest.split_once(' ').unwrap_or((rest, ""));

        match cmd {
            "PRIVMSG"              => { if let Some(m) = parse_privmsg(prefix, params, &tags) { let _ = app.emit("twitch:message", m); } }
            "USERNOTICE"           => { let _ = app.emit("twitch:usernotice", &tags); }
            "CLEARCHAT"|"CLEARMSG" => { let _ = app.emit("twitch:clearchat", line); }
            "RECONNECT"            => { RUNNING.store(false, Ordering::SeqCst); }
            "001"                  => { let _ = app.emit("twitch:ready", ()); }
            _ => {}
        }
    }
}