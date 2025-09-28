use chrono::Local;
use serde::Deserialize;
use std::fs;
use teloxide::{prelude::*, utils::command::BotCommands};

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub mod constants;
pub mod db;
pub mod parsers;
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
        parse_with = parsers::parse_four_delimited_strings
    )]
    FlashCard {
        question: String,
        answer: String,
        topic: String,
        difficulty: u64,
    },
    #[command(
        description = "Register yourself so you can keep track of flshcards and review sessions."
    )]
    Register,
    #[command(description = "Explain a concept.")]
    Explain(String),
    #[command(description = "Define a concept.")]
    Define(String),
    #[command(description = "Translate a text.", parse_with = parsers::parse_two_delimited_strings)]
    Translate { language: String, text: String },
    #[command(description = "Compare 2 concepts", parse_with = parsers::parse_two_delimited_strings)]
    Compare { concept1: String, concept2: String },
    #[command(description = "summarize given text.")]
    Summarize(String),
    // #[command(description = "start a quiz using uploaded flashcards.")]
    // Quiz,
    // #[command(description = "review flashcards using space repetition.")]
    // Review,
}

pub enum Actions {
    Summarize,
    Explain,
    Define,
    Translate,
    Compare,
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

    Command::repl(bot, answer).await;
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Register => {
            let sender = msg.from;

            match sender {
                Some(u) => {
                    // additional block to drop mutex before calling await
                    {
                        let db = db::get_db();
                        let mut statement = db
                            .prepare("INSERT INTO users (id, username, joined_at) VALUES (?,?,?)")
                            .unwrap();

                        statement.bind((1, u.id.to_string().as_str())).unwrap();
                        statement
                            .bind((
                                2,
                                u.username
                                    .unwrap_or(String::from("anonymous_user"))
                                    .as_str(),
                            ))
                            .unwrap();
                        statement
                            .bind((3, Local::now().to_rfc3339().as_str()))
                            .unwrap();
                        while let Ok(sqlite::State::Row) = statement.next() {
                            debug!("id = {}", statement.read::<i64, _>("id").unwrap());
                            debug!(
                                "username = {}",
                                statement.read::<String, _>("username").unwrap()
                            );
                            debug!(
                                "joined on {}",
                                statement.read::<String, _>("joined_at").unwrap()
                            );
                        }
                    }
                    bot.send_message(
                        msg.chat.id,
                        format!("You successfully registered on quiz pal"),
                    )
                    .await?
                }
                None => {
                    bot.send_message(msg.chat.id, format!("Error while processign your request "))
                        .await?
                }
            }
        }
        Command::Summarize(text) => {
            info!("{}", text);
            let result: Result<requests::ModelAnswer, requests::RequestError> =
                requests::request(text.as_str(), Actions::Summarize).await;

            match result {
                Ok(v) => bot.send_message(msg.chat.id, v.content).await?,
                Err(error) => match error {
                    requests::RequestError::Http(e) => {
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
                    }
                },
            }
        }
        Command::Define(text) => {
            let result: Result<requests::ModelAnswer, requests::RequestError> =
                requests::request(text.as_str(), Actions::Define).await;

            match result {
                Ok(v) => bot.send_message(msg.chat.id, v.content).await?,
                Err(error) => match error {
                    requests::RequestError::Http(e) => {
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
                    }
                },
            }
        }
        Command::Translate { language, text } => {
            let result: Result<requests::ModelAnswer, requests::RequestError> = requests::request(
                format!("Translate text in {}\nText: {}\n", language, text).as_str(),
                Actions::Translate,
            )
            .await;

            match result {
                Ok(v) => bot.send_message(msg.chat.id, v.content).await?,
                Err(error) => match error {
                    requests::RequestError::Http(e) => {
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
                    }
                },
            }
        }
        Command::Compare { concept1, concept2 } => {
            let result: Result<requests::ModelAnswer, requests::RequestError> = requests::request(
                format!("First concept {}\nSecond concept: {}\n", concept1, concept2).as_str(),
                Actions::Compare,
            )
            .await;

            match result {
                Ok(v) => bot.send_message(msg.chat.id, v.content).await?,
                Err(error) => match error {
                    requests::RequestError::Http(e) => {
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
                    }
                },
            }
        }
        Command::FlashCard {
            question,
            answer,
            topic,
            difficulty,
        } => {
            let sender = msg.from;

            match sender {
                Some(u) => {
                    {
                        let db = db::get_db();

                        let query = "SELECT id FROM users WHERE id = ?";
                        let mut user_stmt = db.prepare(query).unwrap();
                        user_stmt.bind((1, u.id.0.to_string().as_str())).unwrap();

                        if user_stmt.next().is_ok() {
                            let mut statement = db
                            .prepare("INSERT INTO flashcards (question, answer, topic, difficulty) VALUES (?,?,?,?)")
                            .unwrap();

                            statement.bind((1, question.as_str())).unwrap();
                            statement.bind((2, answer.as_str())).unwrap();
                            statement.bind((3, topic.as_str())).unwrap();
                            statement
                                .bind((4, difficulty.to_string().as_str()))
                                .unwrap();

                            while let Ok(sqlite::State::Row) = statement.next() {
                                debug!(
                                    "question = {}",
                                    statement.read::<String, _>("question").unwrap()
                                );
                                debug!(
                                    "answer = {}",
                                    statement.read::<String, _>("answer").unwrap()
                                );
                                debug!("topic = {}", statement.read::<String, _>("topic").unwrap());
                                debug!(
                                    "difficulty = {}",
                                    statement.read::<i64, _>("difficulty").unwrap()
                                );
                            }
                        } else {
                            error!("user not found");
                        }
                    };

                    bot.send_message(msg.chat.id, format!("You successfully created a flashcard"))
                        .await?
                }
                None => {
                    bot.send_message(
                        msg.chat.id,
                        format!("User ID not available, couldn't create flashcard"),
                    )
                    .await?
                }
            }
        }
    };
    Ok(())
}
