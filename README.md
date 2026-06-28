# xof-fighter

Hello FGC!

## Install

### AUR (Arch Linux)

Stable release (requires an account on aur.archlinux.org):

```bash
yay -S xof-fighter
# or
paru -S xof-fighter
```

Latest git build:

```bash
yay -S xof-fighter-git
# or
paru -S xof-fighter-git
```

Or manually using the PKGBUILDs in this repo:

```bash
git clone https://github.com/Foxashi/xof-fighter.git
cd xof-fighter/pkg/aur-git   # or pkg/aur for the stable release
makepkg -si
```

### From source

**Using Make (recommended — also used for updates):**

```bash
git clone https://github.com/Foxashi/xof-fighter.git
cd xof-fighter
sudo make install
```

To update later:

```bash
cd xof-fighter
sudo make update
```

To uninstall:

```bash
sudo make uninstall
```

**Manually:**

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
