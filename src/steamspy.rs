use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use std::collections::HashMap;

const STEAMSPY_TAG_URL: &str = "https://steamspy.com/api.php?request=tag&tag=Fighting&page=";

#[derive(Deserialize, Debug, Clone)]
pub struct SteamSpyGame {
    pub appid: u32,
    pub name: String,
    pub developer: String,
    /// Peak concurrent users yesterday
    pub ccu: u32,
}

/// Fetch fighting games from SteamSpy.
pub async fn fetch_fighting_games(client: &Client, pages: u8) -> Result<Vec<SteamSpyGame>> {
    let mut all: HashMap<String, SteamSpyGame> = HashMap::new();

    for page in 0..pages {
        let url = format!("{}{}", STEAMSPY_TAG_URL, page);
        let chunk: HashMap<String, SteamSpyGame> =
            client.get(&url).send().await?.json().await?;

        if chunk.is_empty() {
            break; // no more pages
        }
        all.extend(chunk);
    }

    let mut games: Vec<SteamSpyGame> = all.into_values().collect();
    games.sort_by(|a, b| b.ccu.cmp(&a.ccu));
    Ok(games)
}
