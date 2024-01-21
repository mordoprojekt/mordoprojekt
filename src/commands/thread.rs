use serenity::all::{ChannelType, CreateThread};
use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn thread(
    ctx: Context<'_>,
    #[description = "thread name"]
    thread_name: String)
-> Result<(), Error> {
    let handle = ctx.reply("Creating Thread...").await?;
    let thread = CreateThread::new(thread_name).kind(ChannelType::PublicThread);
    let channel = match ctx.guild_channel().await{
      Some(channel) => channel,
      None => {
          ctx.reply("Failed to get Channel").await?;
          return Ok(())
      }
    };
    channel.create_thread(ctx, thread).await?;
    handle.delete(ctx).await?;
    Ok(())
}