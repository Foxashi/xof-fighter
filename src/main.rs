mod display;
mod games;
mod steam;
mod steamspy;

use anyhow::Result;
use clap::Parser;
use reqwest::Client;

#[derive(Parser)]
#[command(
    name = "xof-fighter",
    version,
    about = "Steam player counts for fighting games — powered by SteamSpy"
)]
struct Cli {
    #[arg(short, long, value_name = "NAME")]
    filter: Option<String>,

    /// Show only the top N results by player count
    #[arg(short, long, value_name = "N")]
    top: Option<usize>,

    #[arg(short, long, value_name = "PAGES", default_value_t = 1)]
    pages: u8,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    let client = Client::builder()
        .user_agent(concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION")))
        .build()?;

    println!("Fetching fighting games from SteamSpy (page 1–{})...\n", cli.pages);

    let allowed = games::allowed_ids();
    let mut games = steamspy::fetch_fighting_games(&client, cli.pages).await?;
    games.retain(|g| allowed.contains(&g.appid));

    // Apply name filter
    if let Some(ref filter) = cli.filter {
        let f = filter.to_lowercase();
        games.retain(|g| g.name.to_lowercase().contains(&f));
    }

    if games.is_empty() {
        eprintln!("No games matched the given filters.");
        return Ok(());
    }

    if let Some(n) = cli.top {
        games.truncate(n);
    }

    println!("Fetching live player counts for {} games...\n", games.len());

    let mut set = tokio::task::JoinSet::new();
    for (i, game) in games.iter().enumerate() {
        let client = client.clone();
        let app_id = game.appid;
        set.spawn(async move { (i, steam::fetch_player_count(&client, app_id).await) });
    }

    let mut live: Vec<Option<u32>> = vec![None; games.len()];
    while let Some(res) = set.join_next().await {
        if let Ok((i, Ok(count))) = res {
            live[i] = count;
        }
    }

    println!("Showing {} game(s)\n", games.len());
    display::print_table(games, live);

    Ok(())
}


