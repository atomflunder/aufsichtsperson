use crate::{Context, Error};
use reqwest;
use serde_json::Value;

#[poise::command(slash_command)]

/// Current Apex Map.
pub async fn map(ctx: Context<'_>) -> Result<(), Error> {
    let api_key = include_str!("../../files/key.txt");

    let request = reqwest::get(format!(
        "https://api.mozambiquehe.re/maprotation?auth={}",
        api_key
    ))
    .await?
    .text()
    .await?;

    let v: Value = serde_json::from_str(&*request)?;

    let current_map = (
        &v["current"]["map"].as_str().unwrap(),
        &v["current"]["remainingTimer"].as_str().unwrap(),
        &v["current"]["asset"].as_str().unwrap(),
    );
    let next_map = &v["next"]["map"].as_str().unwrap();

    ctx.send(|m| {
        m.content("Current Apex Map:").embed(|e| {
            e.title(current_map.0)
                .description(format!("**Time remaining:** {}", current_map.1))
                .footer(|f| f.text(format!("Next Up: {}", next_map)))
                .colour((218, 41, 42))
                .image(format!("{}", current_map.2))
        })
    })
    .await?;

    Ok(())
}
