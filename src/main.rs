use serde::Deserialize;
use poise::serenity_prelude as serenity;
use serenity::all::{CreateAttachment, GatewayIntents};
use serenity::client::EventHandler;
use serenity::{async_trait, Client};
use std::fs;
use std::fs::File;
use std::io::Read;
use tokio::sync::Mutex;

#[derive(Deserialize)]
struct ConfigData {
    config: Config,
}
#[derive(Deserialize)]
struct Config {
    token: String,
    prefix: String,
}

struct Attachment {
    data: Vec<u8>,
    filename: String,
}

struct Resources {
    gimper: Attachment,
}

impl serenity::prelude::TypeMapKey for Resources {
    type Value = Resources;
}

pub struct Data {
gimper: Mutex<Attachment>
} // User data, which is stored and accessible in all command invocations
type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

fn read_config() -> Config {
    let config_file = "config.toml";

    let contents = fs::read_to_string(config_file)
        .expect(&format!("Config file: {} not found.", config_file));

    toml::from_str::<ConfigData>(&contents).expect("").config
}

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

struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    let config = read_config();
    let token = config.token;
    let resources = Mutex::new(create_resources().await);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), gimper()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {gimper:resources})
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

#[poise::command(slash_command, prefix_command)]
async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<serenity::User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response = format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn gimper(
    ctx: Context<'_>
) -> Result<(), Error> {
    let gimper = ctx.data().gimper.lock().await;
    let gimper_attachment = CreateAttachment::bytes(
        gimper.data.clone(),
        &gimper.filename,
    );
    let reply = poise::CreateReply::default()
        .attachment(gimper_attachment);

    ctx.send(reply).await?;
    Ok(())
}

/*
#[command]
async fn gimper(ctx: &Context, msg: &Message) -> CommandResult {
    let data = ctx.data.read().await;
    let resources = data.get::<Resources>().unwrap();
    let gimper_attachment = CreateAttachment::bytes(
        resources.gimper.data.clone(),
        &resources.gimper.filename,
    );

    let builder = CreateMessage::new().add_file(gimper_attachment);
    match msg.channel_id.send_message(&ctx.http, builder).await {
        Ok(_) => Ok(()),
        Err(e) => Err(CommandError::from(e)),
    }
}
 */
