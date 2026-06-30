use anyhow::Result;
use reqwest::Client;
use serde::Deserialize;
use std::time::Duration;

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

/// Fetches the current number of players for a Steam app.
/// Retries up to 3 times with exponential backoff on transient errors.
/// Returns `None` if Steam reports the game as unavailable/delisted.
pub async fn fetch_player_count(client: &Client, app_id: u32) -> Result<Option<u32>> {
    let url = format!("{}?appid={}", PLAYER_COUNT_URL, app_id);
    let mut last_err: anyhow::Error = anyhow::anyhow!("no attempts made");

    for attempt in 0..3u32 {
        if attempt > 0 {
            tokio::time::sleep(Duration::from_millis(300 * (1u64 << (attempt - 1)))).await;
        }
        match try_fetch(client, &url).await {
            Ok(result) => return Ok(result),
            Err(e) => last_err = e,
        }
    }
    Err(last_err)
}

async fn try_fetch(client: &Client, url: &str) -> Result<Option<u32>> {
    let resp: ApiResponse = client.get(url).send().await?.json().await?;
    if resp.response.result == 1 {
        Ok(resp.response.player_count)
    } else {
        Ok(None)
    }
}
