use chrono::Local;
use serde::Deserialize;
use std::fs;
use teloxide::{prelude::*, types::ParseMode, utils::command::BotCommands};

use crate::types::FlashCardData;

extern crate pretty_env_logger;
#[macro_use]
extern crate log;

pub mod constants;
pub mod db;
pub mod parsers;
pub mod requests;
pub mod types;
pub mod utils;

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
    #[command(description = "List all falshcards available for a given topic")]
    List(String),
    // #[command(description = "Start a quiz using uploaded flashcards on a specific topic.")]
    // Quiz(String),
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
                Ok(v) => {
                    bot.send_message(msg.chat.id, utils::escape_md_v2(&v.content).as_str())
                        .parse_mode(ParseMode::MarkdownV2)
                        .await?
                }
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
                Ok(v) => {
                    bot.send_message(msg.chat.id, utils::escape_md_v2(&v.content).as_str())
                        .parse_mode(ParseMode::MarkdownV2)
                        .await?
                }
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
                Ok(v) => {
                    bot.send_message(msg.chat.id, utils::escape_md_v2(&v.content).as_str())
                        .parse_mode(ParseMode::MarkdownV2)
                        .await?
                }
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
                            format!(
                                "Error while processign yourconvert char to &str rust request {}",
                                e
                            ),
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
                Ok(v) => {
                    bot.send_message(msg.chat.id, utils::escape_md_v2(&v.content).as_str())
                        .parse_mode(ParseMode::MarkdownV2)
                        .await?
                }
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
                Ok(v) => {
                    bot.send_message(msg.chat.id, utils::escape_md_v2(&v.content).as_str())
                        .parse_mode(ParseMode::MarkdownV2)
                        .await?
                }
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
                    let user_exist = {
                        let db = db::get_db();

                        let query = "SELECT id FROM users WHERE id = ?";
                        let mut user_stmt = db.prepare(query).unwrap();
                        user_stmt.bind((1, u.id.0.to_string().as_str())).unwrap();

                        user_stmt.next().is_ok()
                    };

                    if user_exist {
                        bot.send_message(
                            msg.chat.id,
                            format!("You successfully created a flashcard"),
                        )
                        .await?;
                    }

                    let res: Result<(), sqlite::Error> = {
                        let db = db::get_db();
                        let mut statement = db
                        .prepare("INSERT INTO flashcards (question, answer, topic, difficulty) VALUES (?,?,?,?)")
                        .unwrap();

                        statement.bind((1, question.as_str())).unwrap();
                        statement.bind((2, answer.as_str())).unwrap();
                        statement.bind((3, topic.as_str())).unwrap();
                        statement
                            .bind((4, difficulty.to_string().as_str()))
                            .unwrap();

                        match statement.next() {
                            Ok(sqlite::State::Done) => Ok(()),
                            Ok(sqlite::State::Row) => Ok(()),
                            Err(e) => Err(e),
                        }
                    };

                    if res.is_ok() {
                        bot.send_message(msg.chat.id, format!("Error while creating flashcard"))
                            .await?
                    } else {
                        bot.send_message(
                            msg.chat.id,
                            format!("You successfully created a flashcard"),
                        )
                        .await?
                    }
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
        Command::List(topic) => {
            let sender = msg.from;

            match sender {
                Some(u) => {
                    let cards: Vec<FlashCardData> = {
                        let db = db::get_db();
                        let mut statement = db
                            .prepare(
                                "
                                SELECT question, answer, difficulty
                                WHERE topic = ? AND user_id = ?
                                ORDER BY difficulty
                                ",
                            )
                            .unwrap();

                        statement.bind((1, u.id.to_string().as_str())).unwrap();
                        statement.bind((2, topic.as_str())).unwrap();

                        let mut rows: Vec<types::FlashCardData> = Vec::new();

                        loop {
                            match statement.next() {
                                Ok(sqlite::State::Row) => {
                                    debug!(
                                        "question = {}",
                                        statement.read::<String, _>("question").unwrap()
                                    );
                                    debug!(
                                        "answer = {}",
                                        statement.read::<String, _>("answrr").unwrap()
                                    );
                                    debug!(
                                        "difficulty = {}",
                                        statement.read::<i64, _>("difficulty").unwrap()
                                    );

                                    rows.push(FlashCardData {
                                        question: statement.read::<String, _>("question").unwrap(),
                                        answer: statement.read::<String, _>("question").unwrap(),
                                        difficulty: statement.read::<i64, _>("difficulty").unwrap(),
                                    });
                                }
                                Ok(sqlite::State::Done) => break,
                                Err(_) => break,
                            }
                        }

                        rows
                    };

                    let mut message: String = String::from("");

                    if cards.len() == 0 {
                        bot.send_message(
                            msg.chat.id,
                            format!("No flashcards found for topic {}", topic),
                        )
                        .await?;
                    }

                    for (i, card) in cards.iter().enumerate() {
                        message.push_str(
                            format!(
                                "Flashcard {}\nQuestion:\n{}\nAnswer:\n{}\nDifficulty (1-10): {}",
                                i + 1,
                                card.question,
                                card.answer,
                                card.difficulty
                            )
                            .as_str(),
                        )
                    }
                    bot.send_message(msg.chat.id, message).await?
                }
                None => {
                    bot.send_message(msg.chat.id, format!("Error while processign your request "))
                        .await?
                }
            }
        }
    };
    Ok(())
}
