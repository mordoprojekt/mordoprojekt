use crate::{Context, Error};
use serenity::all::CreateAttachment;

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
