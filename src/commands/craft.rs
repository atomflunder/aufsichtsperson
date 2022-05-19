use crate::{Context, Error};
use reqwest;
use serde_json::Value;
use titlecase::titlecase;

fn hex_to_rgb(hex_colour: String) -> (u8, u8, u8) {
    // The API sends us a Hex Colour Code but we need RGB values.

    let r_value = u8::from_str_radix(&hex_colour[1..=2], 16).unwrap();
    let g_value = u8::from_str_radix(&hex_colour[3..=4], 16).unwrap();
    let b_value = u8::from_str_radix(&hex_colour[5..=6], 16).unwrap();

    return (r_value, g_value, b_value);
}

#[poise::command(slash_command)]

/// Current Crafting Rotation.
pub async fn craft(ctx: Context<'_>) -> Result<(), Error> {
    let api_key = include_str!("../../files/key.txt");

    let request = reqwest::get(format!(
        "https://api.mozambiquehe.re/crafting?auth={}",
        api_key
    ))
    .await?
    .text()
    .await?;

    let v: Value = serde_json::from_str(&*request)?;

    let daily_bundle = (
        (
            titlecase(
                &v[0]["bundleContent"][0]["itemType"]["name"]
                    .as_str()
                    .unwrap()
                    .replace("_", " "),
            ),
            &v[0]["bundleContent"][0]["cost"].to_string(),
            &v[0]["bundleContent"][0]["itemType"]["asset"]
                .as_str()
                .unwrap(),
            &v[0]["bundleContent"][0]["itemType"]["rarity"]
                .as_str()
                .unwrap(),
            &v[0]["bundleContent"][0]["itemType"]["rarityHex"]
                .as_str()
                .unwrap(),
        ),
        (
            titlecase(
                &v[0]["bundleContent"][1]["itemType"]["name"]
                    .as_str()
                    .unwrap()
                    .replace("_", " "),
            ),
            &v[0]["bundleContent"][1]["cost"].to_string(),
            &v[0]["bundleContent"][1]["itemType"]["asset"]
                .as_str()
                .unwrap(),
            &v[0]["bundleContent"][1]["itemType"]["rarity"]
                .as_str()
                .unwrap(),
            &v[0]["bundleContent"][1]["itemType"]["rarityHex"]
                .as_str()
                .unwrap(),
        ),
    );

    let weekly_bundle = (
        (
            titlecase(
                &v[1]["bundleContent"][0]["itemType"]["name"]
                    .as_str()
                    .unwrap()
                    .replace("_", " "),
            ),
            &v[1]["bundleContent"][0]["cost"].to_string(),
            &v[1]["bundleContent"][0]["itemType"]["asset"]
                .as_str()
                .unwrap(),
            &v[1]["bundleContent"][0]["itemType"]["rarity"]
                .as_str()
                .unwrap(),
            &v[1]["bundleContent"][0]["itemType"]["rarityHex"]
                .as_str()
                .unwrap(),
        ),
        (
            titlecase(
                &v[1]["bundleContent"][1]["itemType"]["name"]
                    .as_str()
                    .unwrap()
                    .replace("_", " "),
            ),
            &v[1]["bundleContent"][1]["cost"].to_string(),
            &v[1]["bundleContent"][1]["itemType"]["asset"]
                .as_str()
                .unwrap(),
            &v[1]["bundleContent"][1]["itemType"]["rarity"]
                .as_str()
                .unwrap(),
            &v[1]["bundleContent"][1]["itemType"]["rarityHex"]
                .as_str()
                .unwrap(),
        ),
    );

    ctx.send(|m| {
        m.content("Current Crafting Rotation:")
            .embed(|e| {
                e.title(format!("{} ({})", daily_bundle.0 .0, daily_bundle.0 .3))
                    .description(format!("**Cost:** {}", daily_bundle.0 .1))
                    .thumbnail(daily_bundle.0 .2)
                    .footer(|f| f.text("Daily Rotation"))
                    .colour(hex_to_rgb(daily_bundle.0 .4.to_string()))
            })
            .embed(|e| {
                e.title(format!("{} ({})", daily_bundle.1 .0, daily_bundle.1 .3))
                    .description(format!("**Cost:** {}", daily_bundle.1 .1))
                    .thumbnail(daily_bundle.1 .2)
                    .footer(|f| f.text("Daily Rotation"))
                    .colour(hex_to_rgb(daily_bundle.1 .4.to_string()))
            })
            .embed(|e| {
                e.title(format!("{} ({})", weekly_bundle.0 .0, weekly_bundle.0 .3))
                    .description(format!("**Cost:** {}", weekly_bundle.0 .1))
                    .thumbnail(weekly_bundle.0 .2)
                    .footer(|f| f.text("Weekly Rotation"))
                    .colour(hex_to_rgb(weekly_bundle.0 .4.to_string()))
            })
            .embed(|e| {
                e.title(format!("{} ({})", weekly_bundle.1 .0, weekly_bundle.1 .3))
                    .description(format!("**Cost:** {}", weekly_bundle.1 .1))
                    .thumbnail(weekly_bundle.1 .2)
                    .footer(|f| f.text("Weekly Rotation"))
                    .colour(hex_to_rgb(weekly_bundle.1 .4.to_string()))
            })
    })
    .await?;

    Ok(())
}
