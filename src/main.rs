use teloxide::{prelude::*, utils::command::BotCommands};

pub mod constants;
pub mod requests;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting yout favourite study bot...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await
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
    FlashCard { question: String, answer: String },
    #[command(description = "start a quiz using uploaded flashcards.")]
    Quiz,
    #[command(description = "review flashcards using space repetition.")]
    Review,
    #[command(description = "summarize given text.")]
    Summarize(String),
    #[command(description = "explain a concept.")]
    Explain(String),
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Summarize(text) => {}
        _ => (),
    };
    Ok(())
}
