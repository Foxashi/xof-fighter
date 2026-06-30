use std::collections::HashSet;

/// Returns the set of allowed fighting game App IDs.
/// If `~/.config/xof-fighter/games.txt` exists, it is used instead of the built-in list.
/// The file format is one App ID per line; lines starting with `#` are treated as comments.
pub fn allowed_ids() -> HashSet<u32> {
    if let Some(ids) = load_user_config() {
        if !ids.is_empty() {
            return ids;
        }
    }
    builtin_ids()
}

fn load_user_config() -> Option<HashSet<u32>> {
    let home = std::env::var("HOME").ok()?;
    let path = std::path::PathBuf::from(home)
        .join(".config")
        .join("xof-fighter")
        .join("games.txt");
    let content = std::fs::read_to_string(path).ok()?;
    let ids = content
        .lines()
        .filter_map(|line| {
            let line = line.split('#').next()?.trim();
            if line.is_empty() { None } else { line.parse().ok() }
        })
        .collect();
    Some(ids)
}

fn builtin_ids() -> HashSet<u32> {
    [
        // ── 3D Fighters ────────────────────────────────────────
        1778820, // Tekken 8
        389730,  // Tekken 7
        544750,  // Soulcalibur VI
        838380,  // Dead or Alive 6
        311730,  // Dead or Alive 5 Last Round: Core Fighters
        3112260, // Virtua Fighter 5 R.E.V.O.

        // ── Capcom 2D ──────────────────────────────────────────
        1364780, // Street Fighter 6
        310950,  // Street Fighter V
        45760,   // Ultra Street Fighter IV
        209120,  // Street Fighter X Tekken
        493840,  // Marvel vs. Capcom: Infinite
        357190,  // Ultimate Marvel vs. Capcom 3
        2634890, // MARVEL vs. CAPCOM Fighting Collection
        1685750, // Capcom Fighting Collection
        2400430, // Capcom Fighting Collection 2

        // ── NetherRealm ────────────────────────────────────────
        1971650, // Mortal Kombat 1
        976310,  // Mortal Kombat 11
        627690,  // Injustice 2

        // ── Bandai Namco / others ──────────────────────────────
        577940,  // Killer Instinct

        // ── Arc System Works ───────────────────────────────────
        1384160, // Guilty Gear Strive
        678950,  // Dragon Ball FighterZ
        2176860, // Granblue Fantasy Versus: Rising
        1090630, // Granblue Fantasy Versus
        586140,  // BlazBlue: Centralfiction
        702890,  // BlazBlue: Cross Tag Battle
        263300,  // BlazBlue: Calamity Trigger
        2198830, // Under Night In-Birth II Sys:Celes
        1216060, // DNF Duel
        356910,  // Battle Fantasia -Revised Edition-

        // ── SNK ────────────────────────────────────────────────
        1498570, // The King of Fighters XV
        571260,  // The King of Fighters XIV
        222940,  // The King of Fighters XIII
        222440,  // The King of Fighters 2002 Unlimited Match
        222420,  // The King of Fighters '98 UM Final Edition
        702120,  // The King of Fighters '97 Global Match
        465840,  // The Last Blade
        366240,  // Garou: Mark of the Wolves
        1076830, // Samurai Shodown (2019)
        1076550, // Samurai Shodown V Special
        794580,  // SNK HEROINES Tag Team Frenzy
        2442380, // SNK vs. Capcom: SVC Chaos
        1575670, // SNK vs. Capcom: The Match of the Millennium

        // ── Other notable VS fighters ──────────────────────────
        1372280, // Melty Blood: Type Lumina
        1602010, // Persona 4 Arena Ultimax
        1372110, // JoJo's Bizarre Adventure: All-Star Battle R
        661990,  // Arcana Heart 3 LOVEMAX SIXSTARS!!!!!!
        482450,  // Nitroplus Blasterz: Heroines Infinite Duel
        536560,  // CHAOS CODE -NEW SIGN OF CATASTROPHE-
        871200,  // Fighting EX Layer
        1877020, // Jujutsu Kaisen Cursed Clash
        1999500, // Blazing Strike
        2892130, // Rage of the Dragons NEO
        1786230, // Breakers Collection

        // ── Indie / platform fighters ──────────────────────────
        245170,  // Skullgirls
        383980,  // Rivals of Aether
        2217000, // Rivals of Aether II
        574980,  // Them's Fightin' Herds
        390560,  // Fantasy Strike
        389050,  // Pocket Rumble
        1376070, // Rushdown Revolt
        1420350, // Fraymakers
        725480,  // Slap City
        553310,  // Lethal League Blaze
        261180,  // Lethal League
        244730,  // Divekick
        1264540, // Windjammers 2
        291550,  // Brawlhalla
        1110100, // Power Rangers: Battle for the Grid
        1451190, // Undisputed (boxing)
    ]
    .into()
}

