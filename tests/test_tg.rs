use std::env;
use dotenv;

use rust_telegram_bot::telegram::TelegramBot; 



#[test]
fn can_create_bot() {
    TelegramBot::new("empty_token".to_string());
}


#[tokio::test]
async fn can_get_me() {
    dotenv::dotenv().expect("Can not read dotenv file");

    let bot_token = env::var("TG_BOT_TOKEN")
        .expect("Can not read token");

    let bot = TelegramBot::new(bot_token);
    let result = bot.get_me().await.expect("Can not get bot info");
    
    assert_eq!(result.is_bot, true);
}