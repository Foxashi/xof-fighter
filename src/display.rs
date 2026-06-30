use colored::Colorize;
use serde::Serialize;
use tabled::{
    builder::Builder,
    settings::{object::Columns, Alignment, Modify, Style},
};
use terminal_size::{terminal_size, Width};

use crate::steamspy::SteamSpyGame;

const MIN_NAME: usize = 20;
const MIN_DEV: usize = 12;

fn column_widths(show_appid: bool) -> (usize, usize) {
    let term_w = terminal_size()
        .map(|(Width(w), _)| w as usize)
        .unwrap_or(120);
    // overhead: borders + padding + fixed-width columns
    // 5-col: 6 borders + 10 padding + rank(3) + live(12) + ccu(8) = 39
    // 6-col: 7 borders + 12 padding + rank(3) + live(12) + ccu(8) + appid(7) = 49
    let overhead = if show_appid { 49 } else { 39 };
    let available = term_w.saturating_sub(overhead);
    let name = (available * 6 / 10).max(MIN_NAME);
    let dev  = (available * 4 / 10).max(MIN_DEV);
    (name, dev)
}

fn color_live(live: Option<u32>) -> String {
    match live {
        None              => "—".dimmed().to_string(),
        Some(n) if n >= 1_000 => format_number(n).green().bold().to_string(),
        Some(n) if n >= 100   => format_number(n).yellow().to_string(),
        Some(n)               => format_number(n).dimmed().to_string(),
    }
}

pub fn print_table(games: Vec<SteamSpyGame>, live_counts: Vec<Option<u32>>, show_appid: bool) {
    if games.is_empty() {
        println!("No results.");
        return;
    }

    let (max_name, max_dev) = column_widths(show_appid);
    let mut builder = Builder::default();

    if show_appid {
        builder.push_record([" # ", "Game", "Live Players", "Peak CCU", "Developer", "App ID"]);
    } else {
        builder.push_record([" # ", "Game", "Live Players", "Peak CCU", "Developer"]);
    }

    for (i, (g, &live)) in games.iter().zip(live_counts.iter()).enumerate() {
        let rank   = format!("{}", i + 1);
        let name   = truncate(&g.name, max_name);
        let live_s = color_live(live);
        let ccu_s  = if g.ccu == 0 { "—".dimmed().to_string() } else { format_number(g.ccu) };
        let dev    = truncate(&g.developer, max_dev);

        if show_appid {
            builder.push_record([rank, name, live_s, ccu_s, dev, g.appid.to_string()]);
        } else {
            builder.push_record([rank, name, live_s, ccu_s, dev]);
        }
    }

    let mut table = builder.build();
    table
        .with(Style::modern())
        .with(Modify::new(Columns::single(0)).with(Alignment::right()))
        .with(Modify::new(Columns::single(2)).with(Alignment::right()))
        .with(Modify::new(Columns::single(3)).with(Alignment::right()));

    if show_appid {
        table.with(Modify::new(Columns::single(5)).with(Alignment::right()));
    }

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

#[derive(Serialize)]
struct JsonEntry {
    rank: usize,
    appid: u32,
    name: String,
    developer: String,
    live_players: Option<u32>,
    peak_ccu: u32,
}

pub fn print_json(games: Vec<crate::steamspy::SteamSpyGame>, live_counts: Vec<Option<u32>>) {
    let entries: Vec<JsonEntry> = games
        .into_iter()
        .zip(live_counts)
        .enumerate()
        .map(|(i, (g, live))| JsonEntry {
            rank: i + 1,
            appid: g.appid,
            name: g.name,
            developer: g.developer,
            live_players: live,
            peak_ccu: g.ccu,
        })
        .collect();
    println!("{}", serde_json::to_string_pretty(&entries).unwrap_or_default());
}



