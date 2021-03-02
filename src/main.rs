use dotenv::dotenv;
use futures::StreamExt;
use std::env;
use telegram_bot::{Api, Error, UpdateKind};
use telegram_bot::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    dotenv().ok();
    env_logger::init();

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");

    let api = Api::new(token);

    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            api.send(message.chat.text("Something")).await?;
        }
    }

    Ok(())
}
