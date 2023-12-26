use serde::Deserialize;
use serenity::all::{CreateAttachment, CreateMessage};
use std::fs;
use std::process::exit;

use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{
    CommandError, CommandResult, Configuration, StandardFramework,
};
use serenity::model::channel::Message;
use serenity::prelude::*;

#[derive(Deserialize)]
struct Data {
    config: Config,
}
#[derive(Deserialize)]
struct Config {
    token: String,
    prefix: String,
}

fn read_config() -> Config {
    let config_file = "config.toml";

    let contents = match fs::read_to_string(config_file) {
        Ok(contents) => contents,
        Err(_) => {
            eprintln!("Could not read file {}", config_file);
            exit(1);
        }
    };
    let data: Data = match toml::from_str(&contents) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Invalid config file: {}", e);
            exit(1);
        }
    };

    data.config
}

#[group]
#[commands(ping)]
#[commands(gimper)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let config = read_config();
    let token = config.token;

    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().prefix(config.prefix));

    let intents = GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    match msg.reply(ctx, "Pong!").await {
        Ok(_) => Ok(()),
        Err(e) => Err(CommandError::from(e)),
    }
}

#[command]
async fn gimper(ctx: &Context, msg: &Message) -> CommandResult {
    let builder =
        CreateMessage::new().add_file(CreateAttachment::path("img/gimper.jpg").await.unwrap());
    match msg.channel_id.send_message(&ctx.http, builder).await {
        Ok(_) => Ok(()),
        Err(e) => Err(CommandError::from(e)),
    }
}
