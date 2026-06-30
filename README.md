# xof-fighter

Hello FGC!
The purpose of this CLI tool is to make checking the player count of the most popular fighting games on Steam easier for Linux users.

## Install

Requires [Rust](https://rustup.rs/).

```bash
git clone https://github.com/Foxashi/xof-fighter.git
cd xof-fighter
cargo install --path .
```

This installs the binary to `~/.cargo/bin/xof-fighter`. Make sure `~/.cargo/bin` is in your `PATH`.

To update, pull the latest changes and reinstall:

```bash
cd xof-fighter
git pull
cargo install --path .
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
