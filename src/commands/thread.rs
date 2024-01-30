use crate::{Context, Error};
use serenity::all::{ChannelType, CreateThread};
use serenity::model::id::ChannelId;

#[poise::command(slash_command, prefix_command)]
pub async fn thread(
    ctx: Context<'_>,
    #[description = "thread name"] thread_name: String,
) -> Result<(), Error> {
    let handle = ctx.reply("Creating Thread...").await?;
    let thread = CreateThread::new(thread_name).kind(ChannelType::PublicThread);
    let channel = match ctx.guild_channel().await {
        Some(channel) => channel,
        None => {
            ctx.reply("Failed to get channel").await?;
            return Ok(());
        }
    };

    let guild_channel = channel.create_thread(ctx, thread).await?;

    handle.delete(ctx).await?;
    Ok(())
}
