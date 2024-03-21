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


## Low-level API

1. Telegram Bot с основными методами
2. Telegram Types с описанием всех типов

```rust
use rust_telegram_bot::telegram::TelegramBot;


fn main() {
    let bot = TelegramBot::new("TOKEN");
    bot.send_message(...)
}
```

## High-level API

1. Filters - фильтры, с помощью которых соединяется update и функция
2. Handlers - ??? (а нужно ли)
3. Context - объект, который хранит в себе бота и всякие прЕколы
4. Updater - объект, который обрабатывает все приходящее апдейты