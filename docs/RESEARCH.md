# Research notes

Findings about Steam for Mac and Highball, verified on macOS 26 on Apple Silicon unless noted.

## Where the data lives (no network, no API key)

Paths are relative to `~/Library/Application Support/Steam`.

| Need | Source |
|---|---|
| Owned games | folder names in `appcache/librarycache/<appid>/` |
| Installed, size, update state | `steamapps/appmanifest_*.acf`, plus `libraryfolders.vdf` for extra drives |
| Playtime and last played | `userdata/<id>/config/localconfig.vdf`, `apps.<id>.Playtime` and `LastPlayed` |
| Collections (hand-made groupings) | `userdata/<id>/config/cloudstorage/cloud-storage-namespace-1.json`, keys `user-collections.*` |
| Platforms, controller support, categories, tags, reviews | `appcache/appinfo.vdf` (binary VDF v41 with a string table) |
| Developer, publisher, launch options, branches, cloud | `appinfo.vdf` |
| Cover art | `appcache/librarycache/<appid>/library_600x900.jpg`, CDN fallback |

`appinfo.vdf` entries: header of 16 bytes (magic `0x07564429`, universe, string table offset), then per app `appid`, `size`, 60 bytes of metadata, then a key-value tree whose keys are indices into the string table.

Collections are merged across Steam installs by highest cloud `version` per key. Deleted collections are tombstones that beat older live copies. A game can be in any number of collections.

Steam category ids confirmed against known games: 1 multiplayer, 2 single-player, 9 co-op, 18 partial controller, 24 shared screen, 27 cross-platform, 28 full controller, 36 online PvP, 37 local PvP, 38 online co-op, 39 local co-op, 44 Remote Play Together.

## Highball (Windows games through Wine)

- Each bottle holds a full Windows Steam at `drive_c/Program Files (x86)/Steam` with the same layout as the Mac client.
- Launch link: `highball://play/steam/<appid>?t=<token>`, token from `launch-token` in Highball's data folder.
- `steam.exe -silent -applaunch <appid>` starts Windows Steam with no window; `steam://open/main` forwarded to it shows the window.
- Highball ships an open compatibility database (`db-games/*.json`, keyed by `steam_appid`) with status, renderer and anti-cheat flags.
- The Mac Steam and the Windows Steam in a bottle replace each other's session ("Session Replaced" in `logs/connection_log.txt`).

## Steam startup

- Warm start: about 2.6 s to logged in. Steam logs its own "System startup time".
- Web UI is requested at about 0.3 s and its window exists by about 1 s, in parallel with core startup. The UI is not the critical path.
- Flags found in the client binaries: `-silent`, `-nofriendsui`, `-noverifyfiles`, `-norepairfiles`, `-nobootstrapperupdate`, `-skipinitialbootstrap`, `-applaunch`, `-offlinemode`, `-cef-disable-gpu`, `-cef-disable-site-isolation`. `-no-browser` is not present in this build.
- `-silent -nofriendsui` did not change time or memory measurably (about 2.6 s, about 1.15 GB, 9 processes). The `Steam.app` stub forwards flags to the real client.
- Low Performance Mode only skips the blurred game-page background. Smooth scrolling and GPU web-view toggles are stored as client settings, not in a readable file.
- Steam's web cache (`config/htmlcache/Default/Cache` and `Code Cache`) is safe to clear with Steam closed. Effect on lag is untested.

## Wishlist and prices

- `IWishlistService/GetWishlist/v1?steamid=` works without a key for a public profile and returns appid, priority and date added.
- `store.steampowered.com/api/appdetails?appids=a,b&filters=price_overview&cc=<cc>` accepts several ids and returns initial, final and discount percent.
- `appdetails` for one game returns description, screenshots, trailers, developers, genres and release date.

## UI stack measurements (M3, release builds)

| | Tauri + Svelte | Native egui prototype |
|---|---|---|
| First paint | 582 ms | 200 to 320 ms |
| Memory with covers | about 600 MB | 365 to 480 MB |
| Memory without images | not measured | 119 MB |
| Binary | 10 MB | 5.7 MB |

Images are decoded at full size in both. A thumbnail cache is the first optimization to try.
