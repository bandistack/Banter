use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub channel: String,
    pub username: String,
    pub display_name: String,
    pub text: String,
    pub color: Option<String>,
    pub badges: Vec<String>,
    pub emotes: Option<String>,
    pub id: Option<String>,
    pub ts: Option<String>,
}

pub fn parse_tags(s: &str) -> HashMap<String, String> {
    s.split(';').filter_map(|p| p.split_once('=')).map(|(k, v)| (k.into(), v.into())).collect()
}

pub fn parse_privmsg(prefix: Option<&str>, params: &str, tags: &HashMap<String, String>) -> Option<ChatMessage> {
    let (ch, text) = params.split_once(" :")?;
    let username = prefix.and_then(|p| p.split('!').next()).unwrap_or("?").to_string();
    let tag = |k: &str| tags.get(k).cloned();
    Some(ChatMessage {
        channel:      ch.trim().into(),
        text:         text.into(),
        display_name: tag("display-name").unwrap_or_else(|| username.clone()),
        color:        tag("color").filter(|c| !c.is_empty()),
        badges:       tag("badges").map(|b| b.split(',').filter(|s| !s.is_empty())
                          .map(|s| s.split_once('/').map(|(n, _)| n).unwrap_or(s).into()).collect()).unwrap_or_default(),
        emotes:       tag("emotes"),
        id:           tag("id"),
        ts:           tag("tmi-sent-ts"),
        username,
    })
}