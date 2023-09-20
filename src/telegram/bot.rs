use reqwest::blocking;
use super::types;
use serde_json::Value;
use std::{error::Error, collections::HashMap};


type TgResult<T> = Result<T, Box<dyn Error>>;


#[derive(Debug)]
pub struct TelegramBot {
    base_url: String,
    http_client: blocking::Client,
}


impl TelegramBot {
    pub fn new(telegram_token: String) -> Self {
        let base_url = String::from(
            format!("https://api.telegram.org/bot{}", telegram_token)
        );

        TelegramBot{
            base_url,
            http_client: blocking::Client::new()
        }
    }

    pub fn get_me(&self) -> TgResult<types::User> {
        let url = format!("{}/getMe", self.base_url);
        let response = self.http_client.get(url).send()?;
        response.error_for_status_ref()?;
        let raw_user: Value = response.json()?;
        Ok(serde_json::from_value(raw_user["result"].to_owned())?)
    }

    pub fn get_updates(&self, params: &HashMap<&str, String>) -> TgResult<Vec<types::Update>> {
        let url = format!("{}/getUpdates", self.base_url);
        let response = self.http_client
            .get(url)
            .query(&params)
            .send()?;
        response.error_for_status_ref()?;
        let raw_updates: Value = response.json()?;
        Ok(serde_json::from_value(raw_updates["result"].to_owned())?)
    }

    pub fn send_message(&self, chat_id: i32, text: &String) -> TgResult<types::Message> {
        let url = format!("{}/sendMessage", self.base_url);
        let _chat_id = &chat_id.to_string();

        let mut params = HashMap::new();
        params.insert("chat_id", _chat_id);
        params.insert("text", text);
        let response = self.http_client
            .get(url)
            .query(&params)
            .send()?;
        response.error_for_status_ref()?;
        let raw_message: Value = response.json()?;
        Ok(serde_json::from_value(raw_message["result"].to_owned())?)
    }
}
