use crate::{Context, Error};

#[poise::command(slash_command, guild_only, ephemeral)]

/// Disconnect.
pub async fn stop(ctx: Context<'_>) -> Result<(), Error> {
    let manager = songbird::get(ctx.discord())
        .await
        .expect("Something wrong with Songbird.")
        .clone();

    let bot_channel = if let Some(conn) = manager.get(ctx.guild_id().unwrap()) {
        conn.lock().await.current_channel()
    } else {
        None
    };

    let bot_channel = match bot_channel {
        Some(bot_channel) => bot_channel,
        None => {
            ctx.say("Not in a Voice Channel.").await?;

            return Ok(());
        }
    };

    let author_channel = ctx
        .guild()
        .unwrap()
        .voice_states
        .get(&ctx.author().id)
        .and_then(|v| v.channel_id);

    let author_channel = match author_channel {
        Some(author_channel) => author_channel,
        None => {
            ctx.say("You are not in a Voice Channel.").await?;

            return Ok(());
        }
    };

    if bot_channel.0 != author_channel.0 {
        ctx.say("Not in the right Voice Channel.").await?;

        return Ok(());
    }

    let _leave = manager.leave(ctx.guild_id().unwrap()).await?;

    ctx.say("Disconnected.").await?;

    Ok(())
}
