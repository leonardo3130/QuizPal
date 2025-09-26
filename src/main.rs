use serde::Deserialize;
use std::fs;
use teloxide::{prelude::*, utils::command::BotCommands};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub mod constants;
pub mod requests;

#[derive(Deserialize)]
struct Config {
    tg_key: String,
    llm_key: String,
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(
        description = "create and save a flashcard.",
        parse_with = "split",
        separator = "|"
    )]
    // #[command(description = "upload a flashcard.", separator = "|", s)]
    // FlashCard { question: String, answer: String },
    // #[command(description = "start a quiz using uploaded flashcards.")]
    // Quiz,
    // #[command(description = "review flashcards using space repetition.")]
    // Review,
    #[command(description = "summarize given text.")]
    Summarize(String),
    #[command(description = "explain a concept.")]
    Explain(String),
}

pub enum Actions {
    Summarize,
    Explain,
}

fn load_config() -> Config {
    let config_text = fs::read_to_string("config.toml").expect("Failed to read config.toml");
    toml::from_str(&config_text).expect("Failed to parse config.toml")
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    info!("Starting your favourite study bot...");

    let config = load_config();

    let bot = Bot::new(config.tg_key);

    Command::repl(bot, answer).await
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Summarize(text) => {
            let result: Result<requests::ModelAnswer, requests::RequestError> =
                requests::request(text.as_str(), Actions::Summarize).await;

            match result {
                Ok(v) => bot.send_message(msg.chat.id, v.content).await?,
                Err(error) => match error {
                    requests::RequestError::Http(e) => {
                        // maybe retry, map to StatusCode::BAD_GATEWAY, etc.
                        bot.send_message(
                            msg.chat.id,
                            format!("Error while processign your request {}", e),
                        )
                        .await?
                    }
                    requests::RequestError::Extract(e) => {
                        bot.send_message(
                            msg.chat.id,
                            format!("Error while processign your request {}", e),
                        )
                        .await?
                        // maybe return StatusCode::UNPROCESSABLE_ENTITY, etc.
                    }
                },
            }
        }
        Command::Explain(text) => {
            let result: Result<requests::ModelAnswer, requests::RequestError> =
                requests::request(text.as_str(), Actions::Explain).await;

            match result {
                Ok(v) => bot.send_message(msg.chat.id, v.content).await?,
                Err(error) => match error {
                    requests::RequestError::Http(e) => {
                        // maybe retry, map to StatusCode::BAD_GATEWAY, etc.
                        bot.send_message(
                            msg.chat.id,
                            format!("Error while processign your request {}", e),
                        )
                        .await?
                    }
                    requests::RequestError::Extract(e) => {
                        bot.send_message(
                            msg.chat.id,
                            format!("Error while processign your request {}", e),
                        )
                        .await?
                        // maybe return StatusCode::UNPROCESSABLE_ENTITY, etc.
                    }
                },
            }
        }
    };
    Ok(())
}
