use crate::{Context, Error};
use rand::seq::SliceRandom;
use serde::Deserialize;

#[derive(Deserialize)]
struct ApexChar {
    name: String,
    asset_url: String,
}

#[derive(Deserialize)]
struct ApexChars {
    characters: Vec<ApexChar>,
}

#[poise::command(slash_command)]

/// Random Apex Character.
pub async fn apex(ctx: Context<'_>) -> Result<(), Error> {
    let chars = serde_json::from_str::<ApexChars>(&format!(
        "{}",
        include_str!("../../files/apex_characters.json")
    ))?;

    let chosen_char = chars.characters.choose(&mut rand::thread_rng()).unwrap();

    ctx.send(|m| {
        m.content("Random Apex Character:").embed(|e| {
            e.title(&chosen_char.name)
                .colour((218, 41, 42))
                .image(&chosen_char.asset_url)
        })
    })
    .await?;

    Ok(())
}
