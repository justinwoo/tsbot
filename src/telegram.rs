use serde::{Deserialize, Serialize};

use crate::types::*;

pub fn get_url(token: &String, method_params: &str) -> MyResult<reqwest::Url> {
    let raw_url = format!("https://api.telegram.org/bot{}/{}", token, method_params);
    let parsed_url = reqwest::Url::parse(&raw_url);

    parsed_url.or_else(|e| Err(format!("Failed to parse telegram url: {}", e)))
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Response {
    pub ok: bool,
    pub result: Vec<Update>,
    pub description: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Update {
    pub update_id: u64, // must be used for offset to mark as read
    pub message: Option<Message>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub from: Option<User>,
    pub text: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: u64,
}
