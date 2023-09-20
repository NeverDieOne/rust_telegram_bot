# Telegram Bot

Имплементация TelegramBot API на Rust

## Пример использования

```rust
use rust_telegram_bot::ext::{
    updater::Updater,
    context::Context
};
use rust_telegram_bot::telegram::types::Update;


fn echo(update: &Update, context: &Context) {
    let user_id = update.get_effective_user().id;
    let text = &update.message.text;
    let _ = context.bot.send_message(user_id as i32, text);
}


fn main() {
    let mut updater = Updater::new(String::from("YOUR TOKEN"));
    updater.register_handler(echo);
    updater.start_polling().expect("Something went wrong");
}
```