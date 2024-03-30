use reqwest::Client;
use super::types;
use serde_json::Value;
use std::collections::HashMap;
use anyhow::Result;


#[derive(Debug)]
pub struct TelegramBot {
    base_url: String,
    http_client: Client,
}


impl TelegramBot {
    pub fn new(telegram_token: String) -> Self {
        let base_url = String::from(
            format!("https://api.telegram.org/bot{}", telegram_token)
        );

        Self {
            base_url,
            http_client: Client::new()
        }
    }

    pub async fn get_me(&self) -> Result<types::User> {
        let url = format!("{}/getMe", self.base_url);
        let response = self.http_client.get(url).send().await?;
        response.error_for_status_ref()?;
        let raw_user: Value = response.json().await?;
        Ok(serde_json::from_value(raw_user["result"].to_owned())?)
    }

    pub async fn get_updates(&self, params: &HashMap<&str, String>) -> Result<Vec<types::Update>> {
        let url = format!("{}/getUpdates", self.base_url);
        let response = self.http_client
            .get(url)
            .query(&params)
            .send().await?;
        response.error_for_status_ref()?;
        let raw_updates: Value = response.json().await?;
        Ok(serde_json::from_value(raw_updates["result"].to_owned())?)
    }

    pub async fn send_message(&self, chat_id: i32, text: &String) -> Result<types::Message> {
        let url = format!("{}/sendMessage", self.base_url);
        let _chat_id = &chat_id.to_string();

        let mut params = HashMap::new();
        params.insert("chat_id", _chat_id);
        params.insert("text", text);
        let response = self.http_client
            .get(url)
            .query(&params)
            .send().await?;
        response.error_for_status_ref()?;
        let raw_message: Value = response.json().await?;
        Ok(serde_json::from_value(raw_message["result"].to_owned())?)
    }
}
