use openai_api_rs::v1::chat_completion::{ChatCompletionRequest, self};
use openai_api_rs::v1::common::GPT3_5_TURBO;
use poise::serenity_prelude as serenity;
use serenity::all::{CreateAttachment, GatewayIntents};
use serenity::client::EventHandler;
use serenity::{async_trait, Client};
use std::fs::File;
use std::io::Read;
use std::{env, fs};
use tokio::sync::Mutex;
use openai_api_rs::v1::api::Client as OpenAiClient;

struct Attachment {
    data: Vec<u8>,
    filename: String,
}

pub struct Data {
    gimper: Mutex<Attachment>,
    openai_client: Mutex<OpenAiClient>,
}
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
    let gimper_attachment = Mutex::new(create_resources().await);
    let openai_client = Mutex::new(OpenAiClient::new(env::var("OPENAI_API_KEY").unwrap().to_string()));

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![age(), gimper(), gpt()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(
                    ctx,
                    &framework.options().commands,
                )
                .await?;
                Ok(Data {
                    gimper: gimper_attachment,
                    openai_client
                })
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
    let response =
        format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
async fn gimper(ctx: Context<'_>) -> Result<(), Error> {
    let gimper = ctx.data().gimper.lock().await;
    let gimper_attachment =
        CreateAttachment::bytes(gimper.data.clone(), &gimper.filename);
    let reply = poise::CreateReply::default().attachment(gimper_attachment);

    ctx.send(reply).await?;
    Ok(())
}

#[poise::command(slash_command)]
async fn gpt(ctx: Context<'_>,
#[rest]
    #[description = "prompt"] prompt: String) -> Result<(), Error> {
    let content = prompt;
    // TODO: using global singleton client for now, change to transient or scoped
    let openai_client = ctx.data().openai_client.lock().await;
    let req = ChatCompletionRequest::new(
        GPT3_5_TURBO.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: content.to_string(),
            name: None,
            function_call: None,
        }],
    );

    let result = openai_client.chat_completion(req)?;
    let noresponse = String::from("no response");
    let content = result.choices[0].message.content.to_owned().unwrap_or(noresponse);

    ctx.reply(content).await?;
    Ok(())
}
