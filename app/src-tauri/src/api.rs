use reqwest::{Client, Response};
use std::collections::HashMap;

pub async fn api_call(url: &str, params: &HashMap<String, String>) -> Result<Response, String> {
    Ok(
        Client::new()
            .post(url)
            .form(params)
            .send()
            .await
            .map_err(|e| e.to_string())?
    )
}