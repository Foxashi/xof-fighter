mod display;
mod games;
mod steam;
mod steamspy;

use anyhow::Result;
use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;

#[derive(Parser)]
#[command(
    name = "xof-fighter",
    version,
    about = "Steam player counts for fighting games — powered by SteamSpy"
)]
struct Cli {
    /// Filter results by game name (case-insensitive substring match)
    #[arg(short, long, value_name = "NAME")]
    filter: Option<String>,

    /// Show only the top N results by player count
    #[arg(short, long, value_name = "N")]
    top: Option<usize>,

    /// Number of pages to fetch — each page contains ~100 games [default: 1]
    #[arg(short, long, value_name = "PAGES", default_value_t = 1)]
    pages: u8,

    /// Show the App ID column
    #[arg(long)]
    appid: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let client = Client::builder()
        .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
        .timeout(std::time::Duration::from_secs(10))
        .build()?;

    let pb = ProgressBar::new_spinner();
    pb.set_style(
        ProgressStyle::with_template("{spinner:.cyan} {msg}")
            .unwrap()
    );
    pb.enable_steady_tick(Duration::from_millis(80));
    pb.set_message(format!("Fetching fighting games from SteamSpy (page 1–{})...", cli.pages));

    let allowed = games::allowed_ids();
    let mut games = steamspy::fetch_fighting_games(&client, cli.pages).await?;
    games.retain(|g| allowed.contains(&g.appid));

    // Apply name filter
    if let Some(ref filter) = cli.filter {
        let f = filter.to_lowercase();
        games.retain(|g| g.name.to_lowercase().contains(&f));
    }

    if games.is_empty() {
        pb.finish_and_clear();
        eprintln!("No games matched the given filters.");
        return Ok(());
    }

    if let Some(n) = cli.top {
        games.truncate(n);
    }

    pb.set_message(format!("Fetching live player counts for {} games...", games.len()));

    let sem = Arc::new(Semaphore::new(8));
    let mut set = tokio::task::JoinSet::new();
    for (i, game) in games.iter().enumerate() {
        let client = client.clone();
        let app_id = game.appid;
        let permit = Arc::clone(&sem);
        set.spawn(async move {
            let _permit = permit.acquire_owned().await;
            (i, steam::fetch_player_count(&client, app_id).await)
        });
    }

    let mut live: Vec<Option<u32>> = vec![None; games.len()];
    while let Some(res) = set.join_next().await {
        if let Ok((i, Ok(count))) = res {
            live[i] = count;
        }
    }

    pb.finish_and_clear();
    display::print_table(games, live, cli.appid);

    Ok(())
}


