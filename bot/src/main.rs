use std::fs;
use std::process::exit;
use serenity::json::from_str;
use serde::Deserialize;
use std::env;
use std::fs::read;
use serenity::all::{CreateAttachment, CreateMessage};
use serenity::all::standard::CommandError;

use serenity::async_trait;
use serenity::prelude::*;
use serenity::model::channel::Message;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{StandardFramework, Configuration, CommandResult};

#[derive(Deserialize)]
struct Data{
    config: Config
}
#[derive(Deserialize)]
struct Config{
    token: String
}
fn readConfig() -> Config {
    let configfile = "config.toml";
    let contents = fs::read_to_string(configfile);
    let readcontents = match contents{
        Ok(contents) => contents,
        Err(_) => {eprintln!("Could not read file");exit(1);}
    };
    let data:Data = match toml::from_str(&readcontents){
        Ok(contents) => contents,
        Err(e) => {eprintln!("cokolwiek innego {}", e);exit(1);}
    };
    return data.config;
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
    let framework = StandardFramework::new().group(&GENERAL_GROUP);
    framework.configure(Configuration::new().prefix("~")); // set the bot's prefix to "~"

    // Login with a bot token from the environment
    let token = readConfig().token;
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
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}
#[command]
async fn gimper(ctx: &Context, msg: &Message) -> CommandResult {
    let builder = CreateMessage::new()
        .add_file(CreateAttachment::path("img/gimper.jpg").await.unwrap());
    let msg = msg.channel_id.send_message(&ctx.http, builder).await;

    Ok(())
}