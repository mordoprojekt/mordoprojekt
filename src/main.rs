mod commands;

use daemonizr::{Daemonizr, DaemonizrError, Stderr, Stdout};
use poise::FrameworkOptions;
use serenity::all::GatewayIntents;
use serenity::model::id::ChannelId;
use std::collections::HashSet;

use std::fs::File;
use std::io::{ErrorKind, Read};
use std::{env, fs};
use std::{path::PathBuf, process::exit};
use tokio::sync::Mutex;

use nix::sys::signal::kill as sendSignal;
use nix::sys::signal::Signal::SIGKILL;
use nix::unistd::Pid as nixPid;

use openai_api_rs::v1::api::Client as openAiClient;
use serenity::Client as serenityClient;

// names of env vars holding api keys and tokens
const DISCORD_TOKEN: &'static str = "DISCORD_TOKEN";
const OPENAI_API_KEY: &'static str = "OPENAI_API_KEY";

// deamon mode stuff
const PID_FILE: &'static str = "/tmp/mordoprojekt.pid";
const STDOUT_FILE: &'static str = "/tmp/mordoprojekt.out";
const STDERR_FILE: &'static str = "/tmp/mordoprojekt.err";

// TODO: use proper cmd args framework?
// cmd args
const DAEMONIZE_FLAG: &'static str = "-d";

struct Attachment {
    data: Vec<u8>,
    filename: String,
}

pub struct Data {
    openai_client: Mutex<openAiClient>,
    gimper_attachment: Mutex<Attachment>,
    chat_threads: Mutex<HashSet<ChannelId>>,
}

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, crate::Data, Error>;

fn main() {
    // do we want to run in daemon mode?
    let daemonize = env::args()
        .collect::<Vec<_>>()
        .contains(&DAEMONIZE_FLAG.to_string());

    // we don't want to have more than one instance running at a time
    match Daemonizr::new().pidfile(PathBuf::from(PID_FILE)).search() {
        Ok(pid) => {
            let pid = pid.try_into().unwrap();
            // if we fail to kill already running daemon just crash the app
            sendSignal(nixPid::from_raw(pid), SIGKILL)
                .expect("Failed to kill already running daemon");
        }
        Err(DaemonizrError::NoDaemonFound) => (),
        Err(e) => {
            // TODO: what should we do in this case?
            eprintln!("{}", e)
        }
    }

    // ensure that no one is locking pid file
    match fs::remove_file(PID_FILE) {
        Ok(()) => (),

        // if pid file doesn't exist we just continue
        Err(e) if e.kind() == ErrorKind::NotFound => (),

        Err(e) => {
            eprint!("{}", e);
            exit(1);
        }
    }

    if daemonize {
        match Daemonizr::new()
            .pidfile(PathBuf::from(PID_FILE))
            .stdout(Stdout::Redirect(PathBuf::from(STDOUT_FILE)))
            .stderr(Stderr::Redirect(PathBuf::from(STDERR_FILE)))
            .umask(0o027)
            .expect("invalid umask")
            .spawn()
        {
            Ok(()) => (),
            Err(e) => {
                // we have to crash now
                eprintln!("{}", e);
                exit(1);
            }
        }
    }

    tokio::runtime::Runtime::new()
        .expect(r#"failed to create runtime"#)
        .block_on(bot_main());
}

// TODO: only main function should have logic that crashes the app (remove expects and return error instead)
async fn bot_main() {
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

    // TODO: persistance
    let chat_threads = Mutex::new(HashSet::new());
    let gimper_attachment = Mutex::new(gimper);
    let openai_client =
        Mutex::new(openAiClient::new(openai_api_key.to_string()));

    let app_data = Data {
        gimper_attachment,
        openai_client,
        chat_threads,
    };

    let framework = poise::Framework::builder()
        .options(FrameworkOptions {
            event_handler: |ctx, event, framework, data| {
                Box::pin(event_handler(ctx, event, framework, data))
            },
            commands: vec![
                commands::age::age(),
                commands::gimper::gimper(),
                commands::paintdot::paintdot(),
                commands::gpt::gpt(),
                commands::thread::thread(),
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
    let mut client = serenityClient::builder(discord_token, intents)
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
    // TODO: this should be agnostic to current working directory
    let gimper = Attachment {
        data: get_file_as_byte_vec(&String::from("./img/gimper.jpg"))?,
        filename: String::from("gimper.jpg"),
    };

    Ok(gimper)
}

async fn event_handler(
    ctx: &serenity::client::Context,
    event: &serenity::client::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    match event {
        serenity::client::FullEvent::Message { new_message } => 'handle_new_message: {
            if new_message.author.id == ctx.cache.current_user().id {
                break 'handle_new_message;
            }
            {
                let chat_threads = data.chat_threads.lock().await;
                if !chat_threads.contains(&new_message.channel_id) {
                    break 'handle_new_message;
                }
            }
            new_message.reply(ctx, "mordo").await?;
        }
        _ => {}
    }
    Ok(())
}
