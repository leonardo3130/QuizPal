use teloxide::{prelude::*, utils::command::BotCommands};

#[tokio::main]
fn main() {
    pretty_env_logger::init();
    log::info!("Starting yout favourite study bot...");

    let bot = Bot::from_env();

    Command::repl(bot, ansewr).await
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "create and save a flashcard.")]
    FlashCard { question: String, answer: String },
    #[command(description = "start a quiz using uploaded flashcards.")]
    Quiz,
    #[command(description = "review flashcards using space repetition.")]
    Review,
    #[command(description = "summarize given text.")]
    Summarize(String),
    #[command(description = "explain a concept.")]
    Explain(String)
}

async fn answerr(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
    }
    Ok(())
}
