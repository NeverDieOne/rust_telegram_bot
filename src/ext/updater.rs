use std::collections::HashMap;

use super::super::telegram::{bot, types};
use std::error::Error;

use super::context::Context;


pub struct Updater {
    pub telegram_bot: bot::TelegramBot,
    handlers: Vec<fn(&types::Update, &Context)>
}


impl Updater {
    pub fn new(telegram_token: String) -> Updater {
        Updater {
            telegram_bot: bot::TelegramBot::new(telegram_token),
            handlers: vec![]
        }
    }

    pub fn start_polling(&self) -> Result<(), Box<dyn Error>> {
        let mut offset = 0;

        let context = Context {
            bot: &self.telegram_bot,
        };

        loop {
            let mut params = HashMap::new();
            params.insert("offset", offset.to_string());
            params.insert("timeout", "30".to_string());
            let updates = match self.telegram_bot.get_updates(&params) {
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
            
                for handler in &self.handlers {
                    handler(&update, &context);
                    break
                }
            }
        }
    }

    pub fn register_handler(&mut self, func: fn(&types::Update, &Context)) {
        self.handlers.push(func);
    }
}