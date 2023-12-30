mod commands;

use poise::serenity_prelude as serenity;
use serde::Deserialize;
use serenity::all::{CreateAttachment, GatewayIntents};
use serenity::client::EventHandler;
use serenity::{async_trait, Client};
use std::{fs, env};
use std::fs::File;
use std::io::Read;
use tokio::sync::Mutex;

struct Attachment {
    data: Vec<u8>,
    filename: String,
}

pub struct Data {
    gimper: Mutex<Attachment>,
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let mut f =
        File::open(&filename).expect(&format!("File: {} not found", filename));
    let metadata = fs::metadata(&filename)
        .expect(&format!("Unable to read file: {}", filename));
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer).expect(&format!(
        "Unable to read file: {} buffer overflow",
        filename
    ));

    buffer
}

// registers global app resources
async fn create_resources() -> Attachment {
    let gimper = Attachment {
        data: get_file_as_byte_vec(&String::from("./img/gimper.jpg")),
        filename: String::from("gimper.jpg"),
    };
    return gimper;
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("missing token");
    let resources = Mutex::new(create_resources().await);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![commands::age(), commands::gimper(), commands::paintdot()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(
                    ctx,
                    &framework.options().commands,
                )
                .await?;
                Ok(Data { gimper: resources })
            })
        })
        .build();

    let intents =
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
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