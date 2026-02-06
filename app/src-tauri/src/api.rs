// src-tauri/src/api.rs
use reqwest::{Client, Response};
use std::collections::HashMap;

pub async fn post_form(url: &str, params: &HashMap<String, String>) -> Result<Response, String> {
    let client = Client::new();
    let res: Response = client
        .post(url)
        .form(params)
        .send()
        .await
        .map_err(|e| e.to_string())?;
    Ok(res)
}