mod commands;
use commands::*;

use poise::serenity_prelude as serenity;
use poise::{Framework, FrameworkOptions, PrefixFrameworkOptions};
use songbird::SerenityInit;

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;
pub struct Data {}

#[tokio::main]
async fn main() {
    Framework::build()
        .token(include_str!("../files/token.txt").to_string())
        .intents(serenity::GatewayIntents::all())
        .options(FrameworkOptions {
            commands: vec![
                register::register(),
                apex::apex(),
                play::play(),
                stop::stop(),
                map::map(),
                craft::craft(),
                stats::stats(),
            ],
            listener: |ctx, event, framework, user_data| {
                Box::pin(ready_event(ctx, event, framework, user_data))
            },
            prefix_options: PrefixFrameworkOptions {
                prefix: Some("!".into()),
                ..Default::default()
            },
            ..Default::default()
        })
        .user_data_setup(move |_ctx, _ready, _framework| Box::pin(async move { Ok(Data {}) }))
        .client_settings(|c| c.register_songbird())
        .run()
        .await
        .unwrap();
}

async fn ready_event(
    _ctx: &serenity::Context,
    event: &poise::Event<'_>,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _user_data: &Data,
) -> Result<(), Error> {
    if let poise::Event::Ready { data_about_bot } = event {
        println!(
            "Logged in to Discord as {}#{}! Ctrl+C to exit.",
            data_about_bot.user.name, data_about_bot.user.discriminator
        )
    }

    Ok(())
}
