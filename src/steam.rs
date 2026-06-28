use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;

const PLAYER_COUNT_URL: &str =
    "https://api.steampowered.com/ISteamUserStats/GetNumberOfCurrentPlayers/v1/";

#[derive(Deserialize)]
struct ApiResponse {
    response: ResponseInner,
}

#[derive(Deserialize)]
struct ResponseInner {
    player_count: Option<u32>,
    result: u32,
}

/// fetches the current number of players for a Steam app.
/// returns "none" if Steam reports the game as unavailable/delisted.
pub async fn fetch_player_count(client: &Client, app_id: u32) -> Result<Option<u32>> {
    let url = format!("{}?appid={}", PLAYER_COUNT_URL, app_id);
    let resp = client
        .get(&url)
        .send()
        .await?
        .json::<ApiResponse>()
        .await?;

    if resp.response.result == 1 {
        Ok(resp.response.player_count)
    } else {
        Ok(None)
    }
}
