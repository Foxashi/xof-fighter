use anyhow::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SteamSpyGame {
    pub appid: u32,
    pub name: String,
    pub developer: String,
    /// Peak concurrent users yesterday
    pub ccu: u32,
}

/// Strips everything except alphanumeric characters and hyphens, preventing
/// path traversal (`/`, `..`) and URL injection (`&`, `=`, `#`, etc.).
fn sanitize_tag(tag: &str) -> String {
    tag.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' { c } else { '_' })
        .collect::<String>()
        .to_lowercase()
}

fn cache_path(tag: &str, page: u8) -> Option<std::path::PathBuf> {
    let home = std::env::var("HOME").ok()?;
    let dir = std::path::PathBuf::from(home).join(".cache").join("xof-fighter");
    std::fs::create_dir_all(&dir).ok()?;
    let safe_tag = sanitize_tag(tag);
    // Use Path::join with a plain filename — no separators, no traversal possible
    Some(dir.join(format!("steamspy-{safe_tag}-{page}.json")))
}

fn is_cache_fresh(path: &std::path::Path) -> bool {
    path.metadata()
        .and_then(|m| m.modified())
        .map(|t| t.elapsed().unwrap_or(Duration::MAX) < Duration::from_secs(12 * 3600))
        .unwrap_or(false)
}

/// Fetch games for the given SteamSpy tag, using a 12-hour disk cache.
pub async fn fetch_fighting_games(client: &Client, pages: u8, tag: &str) -> Result<Vec<SteamSpyGame>> {
    let mut all: HashMap<String, SteamSpyGame> = HashMap::new();

    for page in 0..pages {
        // Serve from cache when fresh
        if let Some(path) = cache_path(tag, page) {
            if is_cache_fresh(&path) {
                if let Ok(data) = std::fs::read(&path) {
                    if let Ok(chunk) = serde_json::from_slice::<HashMap<String, SteamSpyGame>>(&data) {
                        if !chunk.is_empty() {
                            all.extend(chunk);
                            continue;
                        }
                    }
                }
            }
        }

        // Fetch from SteamSpy API — use sanitized tag to prevent query-string injection
        let url = format!(
            "https://steamspy.com/api.php?request=tag&tag={}&page={}",
            sanitize_tag(tag), page
        );
        let chunk: HashMap<String, SteamSpyGame> = client
            .get(&url)
            .send()
            .await?
            .json()
            .await?;

        if chunk.is_empty() {
            break;
        }

        // Write to cache
        if let Some(path) = cache_path(tag, page) {
            let _ = std::fs::write(path, serde_json::to_vec(&chunk)?);
        }

        all.extend(chunk);
    }

    let mut games: Vec<SteamSpyGame> = all.into_values().collect();
    games.sort_by(|a, b| b.ccu.cmp(&a.ccu));
    Ok(games)
}
