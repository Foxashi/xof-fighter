mod display;
mod games;
mod steam;
mod steamspy;

use anyhow::Result;
use clap::{Parser, ValueEnum};
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::Client;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Semaphore;

#[derive(Clone, ValueEnum)]
enum SortBy {
    /// Sort by current live player count
    Live,
    /// Sort by peak CCU yesterday (SteamSpy)
    Ccu,
    /// Sort alphabetically by name
    Name,
}

#[derive(Clone, ValueEnum)]
enum OutputFormat {
    /// Pretty terminal table
    Table,
    /// JSON array
    Json,
    /// CSV (comma-separated values)
    Csv,
}

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

    /// Show only the top N results (after sorting)
    #[arg(short, long, value_name = "N")]
    top: Option<usize>,

    /// Number of pages to fetch — each page contains ~100 games
    #[arg(short, long, value_name = "PAGES", default_value_t = 1)]
    pages: u8,

    /// Show the App ID column
    #[arg(long)]
    appid: bool,

    /// Sort results by the given field
    #[arg(long, value_enum, default_value_t = SortBy::Ccu)]
    sort: SortBy,

    /// Output format
    #[arg(long, value_enum, default_value_t = OutputFormat::Table)]
    output: OutputFormat,

    /// Disable colored output (also respects NO_COLOR env var)
    #[arg(long)]
    no_color: bool,

    /// SteamSpy tag to query
    #[arg(long, value_name = "TAG", default_value = "Fighting")]
    tag: String,

    /// Skip fetching live player counts from Steam (faster, offline-friendly)
    #[arg(long)]
    no_live: bool,

    /// Bypass the SteamSpy disk cache and force a fresh fetch
    #[arg(long)]
    refresh: bool,

    /// Only show games with at least N peak CCU (SteamSpy)
    #[arg(long, value_name = "N")]
    min_ccu: Option<u32>,

    /// Only show games with at least N live players (no effect with --no-live)
    #[arg(long, value_name = "N")]
    min_live: Option<u32>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.no_color {
        colored::control::set_override(false);
    }

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
    pb.set_message(format!("Fetching {} games from SteamSpy (page 1–{})...", cli.tag, cli.pages));

    let mut games = steamspy::fetch_fighting_games(&client, cli.pages, &cli.tag, cli.refresh).await?;

    // Only apply the curated allowlist for the default Fighting tag
    if cli.tag.eq_ignore_ascii_case("fighting") {
        let allowed = games::allowed_ids();
        games.retain(|g| allowed.contains(&g.appid));
    }

    // Apply name filter
    if let Some(ref filter) = cli.filter {
        let f = filter.to_lowercase();
        games.retain(|g| g.name.to_lowercase().contains(&f));
    }

    // Apply minimum peak CCU filter
    if let Some(min) = cli.min_ccu {
        games.retain(|g| g.ccu >= min);
    }

    if games.is_empty() {
        pb.finish_and_clear();
        eprintln!("No games matched the given filters.");
        return Ok(());
    }

    let live = if cli.no_live {
        vec![None; games.len()]
    } else {
        pb.set_style(
            ProgressStyle::with_template("{spinner:.cyan} [{bar:30.cyan/blue}] {pos}/{len} live counts fetched")
                .unwrap()
                .progress_chars("=>-"),
        );
        pb.set_length(games.len() as u64);
        pb.set_position(0);

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

        let mut counts: Vec<Option<u32>> = vec![None; games.len()];
        while let Some(res) = set.join_next().await {
            if let Ok((i, Ok(count))) = res {
                counts[i] = count;
            }
            pb.inc(1);
        }
        counts
    };

    // Sort combined entries
    let mut entries: Vec<(steamspy::SteamSpyGame, Option<u32>)> =
        games.into_iter().zip(live).collect();

    // Apply live player count filter
    if let Some(min) = cli.min_live {
        entries.retain(|(_, live)| live.unwrap_or(0) >= min);
    }

    if entries.is_empty() {
        pb.finish_and_clear();
        eprintln!("No games matched the given filters.");
        return Ok(());
    }

    match cli.sort {
        SortBy::Live => entries.sort_by(|a, b| b.1.unwrap_or(0).cmp(&a.1.unwrap_or(0))),
        SortBy::Ccu  => entries.sort_by(|a, b| b.0.ccu.cmp(&a.0.ccu)),
        SortBy::Name => entries.sort_by(|a, b| a.0.name.cmp(&b.0.name)),
    }

    if let Some(n) = cli.top {
        entries.truncate(n);
    }

    let (games, live): (Vec<_>, Vec<_>) = entries.into_iter().unzip();

    pb.finish_and_clear();

    match cli.output {
        OutputFormat::Table => display::print_table(games, live, cli.appid),
        OutputFormat::Json  => display::print_json(games, live),
        OutputFormat::Csv   => display::print_csv(games, live),
    }

    Ok(())
}


