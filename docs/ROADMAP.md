# Roadmap

## Strategy

1. Build and finish the web-based app (Tauri + Svelte). Get every feature working and measured.
2. Then write a hand-rolled native frontend from scratch on the same Rust core, precompiled and optimized like a game, and compare it against the web version.

The Rust core owns all Steam and Highball logic, so the second stage only replaces the frontend.

Why web first: the game page needs Steam's HTML descriptions, a screenshot gallery and video trailers, which a webview renders natively, and CSS makes focus and transition polish cheap. Measured on an M3 (release builds): the web shell painted at about 580 ms and used about 600 MB with three WebKit helper processes; a naive native grid prototype painted its first frame at 200 to 320 ms. Native saves roughly 350 to 400 ms at startup. Images dominate memory in both.

## Performance budgets

- Grid painted under 500 ms from process start (logged as `[timing] grid-painted`).
- Memory under 400 MB for the app and its helper processes.
- 60 fps scrolling with the full library.
- Steam, when needed, starts silently and is ready by the time a game is chosen.

## Milestones

### M1: Web app, browse and launch
- Cover grid, collections, sorting, keyboard and gamepad input (done)
- Thumbnail cache and visible-only image decoding
- Play and Install actions for Mac (`steam://`) and Wine (`highball://`)
- Steam lifecycle: start on demand, wait for login, fall back to Steam's window when login is needed
- Windows-only games through Highball with the token read from its data folder

### M2: Web app, game page and management
- Game page: description, gallery, developer, time played, status, launch options
- Properties: launch options, beta branches, cloud, languages
- Actions: verify files, uninstall, browse local files, open in Highball, copy diagnostics
- Debug and controller panel from Steam and Highball logs
- Download and update progress from the app manifests

### M3: Wishlist and deals
- Public wishlist fetch and price fetch with caching
- Sort by largest savings, best deal, percent off, lowest price

### M4: Time to beat
- HowLongToBeat lookup, cached, with manual override

### M5: Steam lifecycle and performance
- Prewarm Steam when the launcher opens, quit it when the launcher closes if the launcher started it
- Web cache maintenance action
- Test trimming or deprioritizing Steam's helper processes while a game runs
- Handle the Mac and Windows Steam sessions replacing each other

### M6: Native frontend
- Hand-rolled renderer, precompiled and tuned like a game
- Same feature set as the web app, compared on the budgets above

## Open questions
- Does `-applaunch` work on Mac Steam for a cold start?
- Which `steam://` forms for uninstall, properties and controller config work on the current client?
- Offline-play detection is a heuristic only. How should it be shown?
- Download size for games that are not installed.
