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

fn color_rank(rank: usize) -> String {
    let s = rank.to_string();
    match rank {
        1 => s.yellow().bold().to_string(),
        2 => s.white().bold().to_string(),
        3 => s.truecolor(205, 127, 50).to_string(),
        _ => s,
    }
}

/// Returns a short publisher/studio tag derived from the developer name.
fn publisher_tag(developer: &str) -> &'static str {
    let d = developer.to_ascii_lowercase();
    if d.contains("arc system")                      { return "ASW"; }
    if d.contains("capcom")                           { return "CAP"; }
    if d.contains("netherrealm")                      { return "NRS"; }
    if d.contains("snk")                              { return "SNK"; }
    if d.contains("bandai")                           { return "BNE"; }
    if d.contains("sega")                             { return "SEGA"; }
    if d.contains("koei") || d.contains("tecmo")     { return "KT"; }
    if d.contains("lab zero") || d.contains("hidden variables") { return "IND"; }
    ""
}

pub fn print_table(games: Vec<SteamSpyGame>, live_counts: Vec<Option<u32>>, show_appid: bool) {
    if games.is_empty() {
        println!("No results.");
        return;
    }

    let (max_name, max_dev) = column_widths(show_appid);
    let mut builder = Builder::default();

    if show_appid {
        builder.push_record([" # ", "Game", "Players Now", "Peak 24h", "Developer", "App ID"]);
    } else {
        builder.push_record([" # ", "Game", "Players Now", "Peak 24h", "Developer"]);
    }

    for (i, (g, &live)) in games.iter().zip(live_counts.iter()).enumerate() {
        let rank   = color_rank(i + 1);
        let tag    = publisher_tag(&g.developer);
        let name   = if tag.is_empty() {
            truncate(&g.name, max_name)
        } else {
            let max_base = max_name.saturating_sub(tag.len() + 3);
            format!("{} {}", truncate(&g.name, max_base), format!("[{tag}]").dimmed())
        };
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
        "  {} game(s)  ·  {} 1,000+   {} 100–999   {} <100   · tags: publisher studio",
        games.len(),
        "■".green().bold(),
        "■".yellow(),
        "■".dimmed(),
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

pub fn print_csv(games: Vec<crate::steamspy::SteamSpyGame>, live_counts: Vec<Option<u32>>) {
    println!("rank,appid,name,developer,live_players,peak_ccu");
    for (i, (g, live)) in games.iter().zip(live_counts.iter()).enumerate() {
        let live_s = live.map(|n| n.to_string()).unwrap_or_default();
        let name_escaped = g.name.replace('"', "\"\"");
        let dev_escaped  = g.developer.replace('"', "\"\"");
        println!("{},{},\"{}\",\"{}\",{},{}", i + 1, g.appid, name_escaped, dev_escaped, live_s, g.ccu);
    }
}

