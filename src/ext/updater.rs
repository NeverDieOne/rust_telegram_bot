use std::collections::HashMap;

use super::super::telegram::{bot, types};
use anyhow::Result;
use super::filters::Filter;


pub struct Updater {
    pub telegram_bot: bot::TelegramBot,
}


impl Updater {
    pub fn new(telegram_token: String) -> Updater {
        Updater {
            telegram_bot: bot::TelegramBot::new(telegram_token),
        }
    }

    pub async fn start_polling(&self) -> Result<()> {
        let mut offset = 0;
        let mut params = HashMap::new();
        params.insert("timeout", "30".to_string());


        loop {
            params.insert("offset", offset.to_string());

            let updates = match self.telegram_bot.get_updates(&params).await {
                Ok(res) => res,
                Err(err) => {
                    if err.source().unwrap().to_string() == "operation timed out" {
                        continue;
                    }
                    panic!("{err}")
                }
            };

            for update in updates {
                offset = update.update_id + 1;
                println!("{:?}", update);
            }
        }
    }

}