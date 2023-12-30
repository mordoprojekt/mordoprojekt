mod commands;

use serenity::all::GatewayIntents;
use serenity::async_trait;
use serenity::client::EventHandler;
use std::fs::File;
use std::io::Read;
use std::process::exit;
use std::{env, fs};
use tokio::sync::Mutex;

use openai_api_rs::v1::api::Client as OpenAiClient;
use serenity::Client as SerenityClient;

// names of env vars holding api keys and tokens
const DISCORD_TOKEN: &'static str = "DISCORD_TOKEN";
const OPENAI_API_KEY: &'static str = "OPENAI_API_KEY";

struct Attachment {
    data: Vec<u8>,
    filename: String,
}

pub struct Data {
    openai_client: Mutex<OpenAiClient>,
    gimper_attachment: Mutex<Attachment>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, crate::Data, Error>;

struct Handler;
#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let discord_token =
        env::var(DISCORD_TOKEN).expect("failed to read discord token");
    let openai_api_key =
        env::var(OPENAI_API_KEY).expect("failed to read openai key");

    let gimper = match create_gimper_attachment() {
        Ok(gimper) => gimper,
        Err(e) => {
            eprintln!("{}", e);
            exit(1);
        }
    };

    let gimper_attachment = Mutex::new(gimper);
    let openai_client =
        Mutex::new(OpenAiClient::new(openai_api_key.to_string()));

    let app_data = Data {
        gimper_attachment,
        openai_client,
    };

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![
                commands::age(),
                commands::gimper(),
                commands::paintdot(),
                commands::gpt(),
            ],
            ..Default::default()
        })
        .setup(|ctx, _, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(
                    ctx,
                    &framework.options().commands,
                )
                .await?;

                Ok(app_data)
            })
        })
        .build();

    let intents =
        GatewayIntents::non_privileged() | GatewayIntents::MESSAGE_CONTENT;
    let mut client = SerenityClient::builder(discord_token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

fn get_file_as_byte_vec(filename: &String) -> Result<Vec<u8>, Error> {
    let mut f = File::open(&filename)?;
    let metadata = fs::metadata(&filename)?;
    let mut buffer = vec![0; metadata.len() as usize];
    f.read(&mut buffer)?;

    Ok(buffer)
}

fn create_gimper_attachment() -> Result<Attachment, Error> {
    let gimper = Attachment {
        data: get_file_as_byte_vec(&String::from("./img/gimper.jpg"))?,
        filename: String::from("gimper.jpg"),
    };

    Ok(gimper)
}
