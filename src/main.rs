use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, CallbackQuery, Message, MessageKind, MediaKind},
    requests::ResponseResult,
    dispatching::UpdateFilterExt,
};
use std::env;
use once_cell::sync::Lazy;
use dotenvy::dotenv;

static TELEGRAM_TOKEN: Lazy<String> =
    Lazy::new(|| env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN não definido"));

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Initializing CryptoToUnlock...");

    let bot = Bot::new(TELEGRAM_TOKEN.clone());

    let handler = dptree::entry()
        .branch(Update::filter_message().endpoint(handle_message))
        .branch(Update::filter_callback_query().endpoint(handle_callback));

    Dispatcher::builder(bot, handler)
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

async fn handle_message(bot: Bot, msg: Message) -> ResponseResult<()> {
    match &msg.kind {
        MessageKind::Common(common_msg) => {
            match &common_msg.media_kind {
                MediaKind::Text(text_content) => {
                    let text = text_content.text.to_lowercase();

                    match text.as_str() {
                        "/start" => {
                            let keyboard = InlineKeyboardMarkup::new(vec![vec![
                                InlineKeyboardButton::callback("Buy", "buy"),
                                InlineKeyboardButton::callback("Sell", "sell"),
                            ]]);

                            bot.send_message(msg.chat.id, "Hello! What would you like to do today?")
                                .reply_markup(keyboard)
                                .await?;
                        }
                        _ => {
                            bot.send_message(msg.chat.id, "Type /start to begin.").await?;
                        }
                    }
                }
                _ => {
                    bot.send_message(msg.chat.id, "Sorry, this type of media is not supported at the moment.")
                        .await?;
                }
            }
        }
        _ => {
            bot.send_message(msg.chat.id, "Sorry, this type of message is not supported yet.")
                .await?;
        }
    }
    Ok(())
}

async fn handle_callback(bot: Bot, query: CallbackQuery) -> ResponseResult<()> {
    let callback_data = query.data.unwrap_or_default();

    bot.answer_callback_query(query.id)
        .text("Got it! Let's continue.")
        .show_alert(false)
        .await?;

    let response_message = match callback_data.as_str() {
        "buy" => "You selected: Buy. Please provide the details of the order you'd like to place.",
        "sell" => "You selected: Sell. Please enter the client's @username so we can proceed.",
        _ => "Unknown option. Please type /start to begin or select an option from the menu.",
    };

    bot.send_message(query.from.id, response_message).await?;

    Ok(())
}