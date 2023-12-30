use serenity::all::CreateAttachment;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;
use image::{ImageBuffer, RgbImage};

const WIDTH:u32 = 256;
const HEIGHT:u32 = 256;

#[poise::command(slash_command, prefix_command)]
pub async fn age(
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
pub async fn gimper(ctx: Context<'_>) -> Result<(), Error> {
    let gimper = ctx.data().gimper.lock().await;
    let gimper_attachment =
        CreateAttachment::bytes(gimper.data.clone(), &gimper.filename);
    let reply = poise::CreateReply::default().attachment(gimper_attachment);

    ctx.send(reply).await?;
    Ok(())
}

#[poise::command(slash_command, prefix_command)]
pub async fn paintdot(ctx: Context<'_>,
                      #[description = "Współrzędna X"] a: u32,
                      #[description = "Współrzędna Y"] b: u32
) -> Result<(), Error> {
    let mut image: RgbImage = ImageBuffer::new(WIDTH, HEIGHT);
    *image.get_pixel_mut(a, b) = image::Rgb([255,255,255]);
    image.save("./img/output.png").unwrap();
    let image_data = std::fs::read("./img/output.png").unwrap();
    let paintdot_attachment =
        CreateAttachment::bytes(image_data, "output.png");
    let reply = poise::CreateReply::default().attachment(paintdot_attachment);

    ctx.send(reply).await?;
    Ok(())
}