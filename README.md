# xof-fighter

Live Steam player counts for fighting games — a CLI tool for Linux users in the FGC.

## Install

### AUR (Arch Linux)

With an AUR helper like `yay` or `paru`:

```bash
yay -S xof-fighter
```
```bash
paru -S xof-fighter
```

Or manually:

```bash
git clone https://aur.archlinux.org/xof-fighter.git
cd xof-fighter
makepkg -si
```

### From source

```bash
git clone https://github.com/Foxashi/xof-fighter.git
cd xof-fighter
cargo build --release
sudo install -Dm755 target/release/xof-fighter /usr/local/bin/xof-fighter
```

## Usage

```
xof-fighter [OPTIONS]

Options:
  -f, --filter <NAME>    Filter results by game name
  -t, --top <N>          Show only the top N results by player count
  -p, --pages <PAGES>    Number of SteamSpy pages to fetch [default: 1]
  -h, --help             Print help
  -V, --version          Print version
```

### Examples

```bash
# Show all tracked fighting games
xof-fighter

# Search for a specific game
xof-fighter --filter "tekken"

# Show top 5 most-played
xof-fighter --top 5

# Fetch more data (more pages = more games)
xof-fighter --pages 3
```

## License

MIT — see [LICENSE](LICENSE)
