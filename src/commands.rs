use serenity::all::CreateAttachment;
use crate::{Context, Error};
use poise::serenity_prelude as serenity;

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