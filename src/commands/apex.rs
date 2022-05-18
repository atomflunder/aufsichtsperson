use crate::{Context, Error};
use rand::seq::SliceRandom;

#[poise::command(slash_command)]

/// Random Apex Character.
pub async fn apex(ctx: Context<'_>) -> Result<(), Error> {
    let apex_chars = include_str!("./../../files/apex_characters.json")
        .lines()
        .filter(|c| c != &"{" && c != &"}")
        .collect::<Vec<&str>>();

    let mut result = apex_chars
        .choose(&mut rand::thread_rng())
        .unwrap()
        .split(": ");

    // We could use some external library to parse json files, but this works good enough.
    let character = result.next().unwrap().replace("\"", "");
    let render = result.next().unwrap().replace("\"", "").replace(",", "");

    ctx.send(|m| {
        m.content("Random Apex Character:")
            .embed(|e| e.title(character).colour((218, 41, 42)).image(render))
    })
    .await?;

    Ok(())
}
