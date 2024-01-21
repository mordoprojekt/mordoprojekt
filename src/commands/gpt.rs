use crate::{Context, Error};
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT3_5_TURBO;
use poise::CreateReply;
use serenity::all::{Colour, CreateEmbed};

const MESSAGE_SIZE_LIMIT: usize = 4096;
#[poise::command(slash_command)]
pub async fn gpt(
    ctx: Context<'_>,
    #[rest]
    #[description = "prompt"]
    prompt: String,
) -> Result<(), Error> {
    // discord complains if we don't reply within 3 seconds
    let handle = ctx.reply("🤔").await?;

    let openai_client = ctx.data().openai_client.lock().await;
    let request = ChatCompletionRequest::new(
        GPT3_5_TURBO.to_string(),
        vec![chat_completion::ChatCompletionMessage {
            role: chat_completion::MessageRole::user,
            content: prompt.to_string(),
            name: None,
            function_call: None,
        }],
    );

    // call to openai takes forever
    let result = openai_client.chat_completion(request)?;
    let content = result.choices[0]
        .message
        .content
        .to_owned()
        .unwrap_or("¯\\_(ツ)_/¯".to_string())
        .chars()
        .take(MESSAGE_SIZE_LIMIT)
        .collect::<String>();

    let message_prompt = format!(">>> {}", prompt);

    let embed = CreateEmbed::default()
        .description(content)
        .color(Colour::RED);
    // edit previous message to include actual response
    let create_reply = CreateReply::default().embed(embed).content(message_prompt);
    handle.edit(ctx, create_reply).await?;
    Ok(())
}