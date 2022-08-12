use crate::{Context, Error};
use songbird::input::Restartable;

#[poise::command(slash_command, guild_only, ephemeral)]

/// Aufsichtsperson Theme.
pub async fn play(ctx: Context<'_>) -> Result<(), Error> {
    let channel_id = ctx
        .guild()
        .unwrap()
        .voice_states
        .get(&ctx.author().id)
        .and_then(|v| v.channel_id);

    let connection = match channel_id {
        Some(channel) => channel,
        None => {
            ctx.say("Voice Channel not found.").await?;

            return Ok(());
        }
    };

    let manager = songbird::get(ctx.discord())
        .await
        .expect("Something wrong with Songbird.")
        .clone();

    let _join = manager.join(ctx.guild_id().unwrap(), connection).await;

    let theme = match Restartable::ffmpeg("./files/theme.mp3", false).await {
        Ok(theme) => theme,
        Err(why) => {
            println!("Error: {}", why);
            return Ok(());
        }
    };

    let _track = manager
        .get(ctx.guild_id().unwrap())
        .unwrap()
        .lock()
        .await
        .play_only_source(theme.into())
        .enable_loop();

    ctx.say("Playing... /stop to stop.").await?;

    Ok(())
}
