use crate::{Context, Error};

use poise::CreateReply;
use serenity::all::{Colour, CreateAttachment, CreateEmbed, User};

use image::{ImageBuffer, RgbImage};
use openai_api_rs::v1::chat_completion::{self, ChatCompletionRequest};
use openai_api_rs::v1::common::GPT3_5_TURBO;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

const MESSAGE_SIZE_LIMIT: usize = 2000;

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
        .collect::<String>();

    let embed = CreateEmbed::default()
        .title(prompt)
        .description(content)
        .color(Colour::BLUE);

    // edit previous message to include actual response
    let create_reply = CreateReply::default().embed(embed);
    handle.edit(ctx, create_reply).await?;
add 
    Ok(())
}

#[poise::command(prefix_command, slash_command)]
pub async fn send_embed(ctx: Context<'_>, #[description = "Embed title"] title: String) -> Result<(), Error> {
    // Create an embed using the CreateEmbed struct
    let embed = CreateEmbed::default()
        .title(title)
        .description("Lorem ipsum dolor sit amet, consectetur adipiscing elit. Aliquam malesuada dictum porttitor. Donec facilisis est a lorem dapibus convallis. Suspendisse commodo, est eget ornare ultricies, mi nisl ornare nisl, quis suscipit augue quam non dolor. Maecenas interdum mi et gravida egestas.\n
Sed libero nisi, gravida ac dictum at, lacinia in tortor. Phasellus erat tellus, egestas non arcu at, iaculis facilisis erat. Nullam interdum erat ut neque rutrum consectetur. Aenean quis pretium sapien. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Vestibulum in consequat ex, non ultricies odio. Suspendisse iaculis commodo orci non cursus. In at risus tempor, congue mauris eu, volutpat odio. Curabitur hendrerit ac libero et iaculis. Mauris vel purus semper, iaculis dolor vel, cursus tortor. In volutpat eros eleifend, congue dui nec, mattis velit.\n
Morbi aliquam mollis ante, mollis porttitor leo placerat eu. Nullam neque neque, dictum vitae tincidunt bibendum, aliquam vitae leo. Phasellus ut dolor cursus ante ornare varius eu sed leo. Fusce quis dignissim ligula. Praesent id maximus nulla. Ut iaculis ultricies nisl, ac tincidunt arcu consectetur vitae. Nunc posuere egestas enim, sit amet tempor ex auctor vel. Etiam erat ipsum, tristique at suscipit non, blandit et nisi.\n
Vestibulum semper neque eget ornare pellentesque. In risus nisi, hendrerit eu rutrum vel, sagittis sit amet ante. Duis rutrum enim non ante aliquet, id suscipit massa fermentum. Maecenas id ullamcorper nulla. Sed ut eleifend justo. Fusce ut cursus neque. Suspendisse viverra odio nisi, et laoreet felis fermentum in.\n
Donec vel libero sed ligula condimentum ultrices. Cras faucibus condimentum quam, nec placerat ipsum rutrum molestie. Pellentesque habitant morbi tristique senectus et netus et malesuada fames ac turpis egestas. Morbi et tincidunt lorem, ac imperdiet massa. Phasellus dolor sem, pulvinar et quam vel, bibendum bibendum elit. Proin nunc lacus, sodales sit amet vestibulum sed, elementum aliquet nisi. Cras venenatis erat magna, at ultrices libero faucibus nec. Curabitur ligula mauris, efficitur et ante vitae, finibus semper ligula. Mauris non nisl felis. Ut pulvinar nisl sit amet lectus interdum posuere. Aliquam erat volutpat. Etiam pretium sagittis nunc a viverra. Nullam pharetra porttitor lacus, eget interdum urna ultrices in.\n
Donec sagittis gravida neque, sit amet elementum turpis laoreet sed. Suspendisse neque dolor, maximus imperdiet libero sed, euismod luctus elit. Proin vel nisl vel sem eleifend condimentum at nec tortor. Cras ut pellentesque erat. Suspendisse eros neque, consectetur in ex non, ullamcorper commodo orci. Donec ac nisi at sapien tempus aliquam vitae eget purus. Vestibulum viverra orci at justo finibus posuere.
")
        .color(Colour::BLUE);
    let reply = CreateReply::default().embed(embed);

    ctx.send(reply).await?;

    Ok(())
}
