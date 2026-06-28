use tabled::{
    settings::{object::Columns, Alignment, Modify, Style},
    Table, Tabled,
};
use terminal_size::{terminal_size, Width};

use crate::steamspy::SteamSpyGame;

/// Overhead in characters taken up by borders, padding, and fixed-width columns:
///   7 borders (│) + 12 padding spaces (6 cols × 2) + fixed content: rank(3) + live(12) + ccu(8) + appid(7) = 49
const FIXED_OVERHEAD: usize = 49;
const MIN_NAME: usize = 20;
const MIN_DEV: usize = 12;

fn column_widths() -> (usize, usize) {
    let term_w = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(120);
    let available = term_w.saturating_sub(FIXED_OVERHEAD);
    // name gets ~60 %, developer the remaining ~40 %
    let name = (available * 6 / 10).max(MIN_NAME);
    let dev  = (available * 4 / 10).max(MIN_DEV);
    (name, dev)
}

#[derive(Tabled)]
struct Row {
    #[tabled(rename = " # ")]
    rank: String,
    #[tabled(rename = "Game")]
    name: String,
    #[tabled(rename = "Live Players")]
    live: String,
    #[tabled(rename = "Peak CCU")]
    ccu: String,
    #[tabled(rename = "Developer")]
    developer: String,
    #[tabled(rename = "App ID")]
    app_id: String,
}

pub fn print_table(games: Vec<SteamSpyGame>, live_counts: Vec<Option<u32>>) {
    if games.is_empty() {
        println!("No results.");
        return;
    }

    let (max_name, max_dev) = column_widths();

    let rows: Vec<Row> = games
        .iter()
        .zip(live_counts.iter())
        .enumerate()
        .map(|(i, (g, &live))| Row {
            rank: format!("{}", i + 1),
            name: truncate(&g.name, max_name),
            live: live.map(format_number).unwrap_or_else(|| "—".to_string()),
            ccu: if g.ccu == 0 {
                "—".to_string()
            } else {
                format_number(g.ccu)
            },
            developer: truncate(&g.developer, max_dev),
            app_id: g.appid.to_string(),
        })
        .collect();

    let mut table = Table::new(&rows);
    table
        .with(Style::modern())
        .with(Modify::new(Columns::single(0)).with(Alignment::right()))
        .with(Modify::new(Columns::single(2)).with(Alignment::right()))
        .with(Modify::new(Columns::single(3)).with(Alignment::right()))
        .with(Modify::new(Columns::single(5)).with(Alignment::right()));

    println!("{table}");
    println!(
        "  {} game(s) — Live = current players now  |  Peak CCU = peak yesterday (SteamSpy)",
        games.len()
    );
}

fn truncate(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        format!("{}…", s.chars().take(max - 1).collect::<String>())
    }
}

fn format_number(n: u32) -> String {
    let s = n.to_string();
    let mut out = String::with_capacity(s.len() + s.len() / 3);
    for (i, ch) in s.chars().rev().enumerate() {
        if i != 0 && i % 3 == 0 {
            out.push(',');
        }
        out.push(ch);
    }
    out.chars().rev().collect()
}


