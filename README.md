# Shelf

A fast, minimal, console-style launcher for your Steam library on macOS. It reads the files Steam already keeps on disk, shows your library as cover art, and launches games through Steam for Mac or, for Windows-only games, through Highball (Wine).

Steam's own client takes seconds to open and is mostly store, community and promotion. Shelf drops all of that and keeps what you use most: browse your own collections, see what can run, launch.

## Goals

- Cover-art grid only, browsed by your own Steam collections plus All and Installed.
- Works with a controller in a normal window, no Big Picture mode.
- At-a-glance constraints per game: native on Mac, Windows-only via Wine, installed, input type, local and online multiplayer.
- A game page with description, gallery, time played, launch options, status, and the management actions Steam hides in right-click menus.
- Sorts Steam lacks: last played, time played, rating, release date, size, savings and best deals on the wishlist, time to beat.
- Starts Steam (or Highball's Windows Steam) only when needed.
- Two stages: finish a web-based app first, then build a hand-written native copy tuned like a game.

See [docs/ROADMAP.md](docs/ROADMAP.md) for milestones and [docs/RESEARCH.md](docs/RESEARCH.md) for what was learned about Steam and Highball.

## Status

Early. The library loads and the grid, collection tabs, sorting, keyboard and gamepad navigation work. Launching, the game page, wishlist and Steam lifecycle are not built yet. Track progress in the issues and milestones.

## Layout

| Path | What it is |
|---|---|
| `crates/shelf-core` | Rust library: Steam file parsers, multi-install merge, collections, features, launch URLs, art, catalog |
| `crates/shelf-spike` | Command line tool that prints what the core reads from your machine |
| `app` | Tauri 2 app with a Svelte frontend |
| `scripts` | Measurement and maintenance scripts |
| `docs` | Roadmap, research, engineering rules |

## Run

Requires Apple Silicon macOS, Steam for Mac, Node, and rustup (the toolchain is pinned in `rust-toolchain.toml`). Highball is optional.

```sh
cargo run -p shelf-spike --release -- summary
cd app && npm install && npm run tauri dev
cargo test
```

## Engineering rules

See [docs/ENGINEERING.md](docs/ENGINEERING.md).
