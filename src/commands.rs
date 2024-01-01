use crate::{Context, Error};

use poise::CreateReply;
use serenity::all::{Colour, CreateAttachment, CreateEmbed, User};

use image::{ImageBuffer, RgbImage};
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT3_5_TURBO;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

const MESSAGE_SIZE_LIMIT: usize = 4096;

#[poise::command(slash_command, prefix_command)]
pub async fn age(
    ctx: Context<'_>,
    #[description = "Selected user"] user: Option<User>,
) -> Result<(), Error> {
    let u = user.as_ref().unwrap_or_else(|| ctx.author());
    let response =
        format!("{}'s account was created at {}", u.name, u.created_at());
    ctx.say(response).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn gimper(ctx: Context<'_>) -> Result<(), Error> {
    let gimper_attachment = ctx.data().gimper_attachment.lock().await;
    let attachment = CreateAttachment::bytes(
        gimper_attachment.data.clone(),
        &gimper_attachment.filename,
    );
    let reply = poise::CreateReply::default().attachment(attachment);

    ctx.send(reply).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn paintdot(
    ctx: Context<'_>,
    #[description = "Współrzędna X"] a: u32,
    #[description = "Współrzędna Y"] b: u32,
) -> Result<(), Error> {
    let mut image: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    *image.get_pixel_mut(a, b) = image::Rgb([255, 255, 255]);
    image.save("./img/output.png").unwrap();
    let image_data = std::fs::read("./img/output.png").unwrap();
    let paintdot_attachment = CreateAttachment::bytes(image_data, "output.png");
    let reply = CreateReply::default().attachment(paintdot_attachment);

    ctx.send(reply).await?;
    Ok(())
}

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

    let embed = CreateEmbed::default()
        .title(prompt)
        .description(content)
        .color(Colour::RED);
    // edit previous message to include actual response
    let create_reply = CreateReply::default().embed(embed).content("");
    handle.edit(ctx, create_reply).await?;
    Ok(())
}
