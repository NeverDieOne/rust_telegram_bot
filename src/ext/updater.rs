use std::collections::HashMap;

use super::super::telegram::{bot, types};
use std::error::Error;

use super::context::Context;
use super::filters::Filter;


pub struct Updater {
    pub telegram_bot: bot::TelegramBot,
    handlers: Vec<(Box<dyn Filter>, fn(&types::Update, &Context))>
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
        let mut params = HashMap::new();
        params.insert("timeout", "30".to_string());

        let context = Context {
            bot: &self.telegram_bot,
        };

        loop {
            params.insert("offset", offset.to_string());
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
                self.handle_update(&update, &context);
            }
        }
    }

    fn handle_update(&self, update: &types::Update, context: &Context) {
        for handler in &self.handlers {
            let filter = &handler.0;
            let handle_func = handler.1;
            if filter.check_update(update) {
                handle_func(update, context);
            }
        }
    }

    pub fn register_handler(&mut self, filter: Box<dyn Filter>, func: fn(&types::Update, &Context)) {
        self.handlers.push((filter, func));
    }
}