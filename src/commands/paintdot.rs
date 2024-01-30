use crate::{Context, Error};
use image::{ImageBuffer, RgbImage};
use poise::CreateReply;
use serenity::all::CreateAttachment;

const WIDTH: u32 = 256;
const HEIGHT: u32 = 256;

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
