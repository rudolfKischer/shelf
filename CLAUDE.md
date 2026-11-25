# Shelf

Console-style Steam launcher for macOS. Rust core in `crates/shelf-core`, Tauri + Svelte app in `app`.

Follow [docs/ENGINEERING.md](docs/ENGINEERING.md) for every code change: no comments, small functions and files, all values in config or JSON, no magic numbers, early returns.

Useful commands:

- `cargo test` runs all tests.
- `cargo run -p shelf-spike --release -- summary` prints what the core reads from this machine.
- `cd app && npm run tauri dev` runs the app.
- Startup timing is logged as `[timing] ...` lines on stderr.

The Rust toolchain is pinned in `rust-toolchain.toml`. Roadmap: `docs/ROADMAP.md`. Findings about Steam and Highball: `docs/RESEARCH.md`.
