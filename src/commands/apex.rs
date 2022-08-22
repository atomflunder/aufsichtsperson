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
    let chars =
        &serde_json::from_str::<ApexChars>(include_str!("../../files/apex_characters.json"))?;

    let invalid_char = ApexChar {
        name: "Invalid Char".to_string(),
        asset_url: "".to_string(),
    };

    let chosen_char = chars
        .characters
        .choose(&mut rand::thread_rng())
        .unwrap_or(&invalid_char);

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
