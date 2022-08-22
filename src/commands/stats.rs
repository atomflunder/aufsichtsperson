use crate::{Context, Error};
use poise::serenity_prelude::CreateEmbed;
use reqwest;
use serde_json::Value::{self, Null};

/// Gets a players stats.
#[poise::command(slash_command, guild_only)]
pub async fn stats(
    ctx: Context<'_>,
    #[description = "The name of the player's Origin Account"] player: String,
) -> Result<(), Error> {
    let api_key = include_str!("../../files/key.txt");

    let request = reqwest::get(format!(
        "https://api.mozambiquehe.re/bridge?auth={}&player={}&platform=PC",
        api_key, player
    ))
    .await?
    .text()
    .await?;

    let v: Value = serde_json::from_str(&*request)?;

    if v["Error"] != Null {
        ctx.send(|m| m.content("Player not found!")).await?;
        return Ok(());
    }

    let name = &v["global"]["name"].as_str().unwrap_or("Invalid Name");
    let id = &v["global"]["uid"].to_string();

    let level = &v["global"]["level"].to_string();
    let percent = &v["global"]["toNextLevelPercent"].to_string();

    let update_count = &v["global"]["internalUpdateCount"].to_string();

    let rank = &v["global"]["rank"]["rankName"]
        .as_str()
        .unwrap_or("Unranked");
    let division = &v["global"]["rank"]["rankDiv"].to_string();

    let rank_image = &v["global"]["rank"]["rankImg"].as_str().unwrap_or("");
    let online_status = &v["realtime"]["currentStateAsText"]
        .as_str()
        .unwrap_or("Offline");

    let selected_legend = &v["legends"]["selected"]["LegendName"]
        .as_str()
        .unwrap_or("Invalid Legend");
    let selected_image = &v["legends"]["selected"]["ImgAssets"]["icon"]
        .as_str()
        .unwrap_or("");

    let tracker1 = &v["legends"]["selected"]["data"][0];
    let tracker2 = &v["legends"]["selected"]["data"][1];
    let tracker3 = &v["legends"]["selected"]["data"][2];

    let mut embed = CreateEmbed::default();

    embed.title(format!("Statistics of {} ({}) (PC)", name, id));
    embed.field(
        format!("Level {}", level),
        format!("To next: {}%", percent),
        false,
    );
    embed.field("Update Count", update_count, false);
    embed.field("BR Rank", format!("{} {}", rank, division), false);
    embed.field("Online status", online_status, false);
    embed.field("Selected Legend", selected_legend, false);

    if tracker1 != &Null {
        embed.field(
            tracker1["name"].as_str().unwrap_or("\u{200b}"),
            tracker1["value"].to_string(),
            true,
        );
    }

    if tracker2 != &Null {
        embed.field(
            tracker2["name"].as_str().unwrap_or("\u{200b}"),
            tracker2["value"].to_string(),
            true,
        );
    }

    if tracker3 != &Null {
        embed.field(
            tracker3["name"].as_str().unwrap_or("\u{200b}"),
            tracker3["value"].to_string(),
            true,
        );
    }

    embed.colour((218, 41, 42));
    embed.image(selected_image);
    embed.thumbnail(rank_image);

    ctx.send(|m| {
        m.embed(|mut e| {
            e.0 = embed.0;
            e
        })
    })
    .await?;

    Ok(())
}
