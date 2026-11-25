use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

use crate::app_info::GameMetadata;
use crate::collections::{merge_newest, Collection};
use crate::config::Config;
use crate::install_snapshot::{load_install, InstallSnapshot, LibraryError};
use crate::local_config::PlayStats;
use crate::manifests::InstalledGame;
use crate::steam_install::{discover_installs, home_dir, Runtime};

#[derive(Debug)]
pub struct InstalledEntry {
    pub runtime: Runtime,
    pub game: InstalledGame,
}

#[derive(Debug)]
pub struct InstallProblem {
    pub runtime: Runtime,
    pub error: LibraryError,
}

#[derive(Debug)]
pub struct LibrarySnapshot {
    pub runtimes: Vec<Runtime>,
    pub library_caches: Vec<PathBuf>,
    pub games: Vec<GameMetadata>,
    pub installed: Vec<InstalledEntry>,
    pub collections: Vec<Collection>,
    pub play_stats: HashMap<u32, PlayStats>,
    pub problems: Vec<InstallProblem>,
}

pub fn load_snapshot(config: &Config) -> LibrarySnapshot {
    let installs = home_dir()
        .map(|home| discover_installs(&home, config))
        .unwrap_or_default();
    let (snapshots, problems) = load_all(&installs, config);
    merge_snapshots(snapshots, problems, config)
}

fn load_all(
    installs: &[crate::steam_install::SteamInstall],
    config: &Config,
) -> (Vec<InstallSnapshot>, Vec<InstallProblem>) {
    let mut snapshots = Vec::new();
    let mut problems = Vec::new();
    for install in installs {
        match load_install(install, config) {
            Ok(snapshot) => snapshots.push(snapshot),
            Err(error) => problems.push(InstallProblem {
                runtime: install.runtime.clone(),
                error,
            }),
        }
    }
    (snapshots, problems)
}

fn merge_snapshots(
    snapshots: Vec<InstallSnapshot>,
    problems: Vec<InstallProblem>,
    config: &Config,
) -> LibrarySnapshot {
    let runtimes = snapshots
        .iter()
        .map(|s| s.install.runtime.clone())
        .collect();
    let library_caches = snapshots
        .iter()
        .map(|s| s.install.root.join(&config.library_cache_dir))
        .collect();
    let play_stats = merge_play_stats(snapshots.iter().map(|s| &s.play_stats));
    let mut installed = Vec::new();
    let mut collection_sets = Vec::new();
    let mut games = Vec::new();
    for snapshot in snapshots {
        let runtime = snapshot.install.runtime;
        installed.extend(snapshot.installed.into_iter().map(|game| InstalledEntry {
            runtime: runtime.clone(),
            game,
        }));
        collection_sets.push(snapshot.collections);
        games.push(snapshot.games);
    }
    LibrarySnapshot {
        runtimes,
        library_caches,
        games: merge_games(games),
        installed,
        collections: merge_newest(collection_sets),
        play_stats,
        problems,
    }
}

fn merge_games(sets: Vec<Vec<GameMetadata>>) -> Vec<GameMetadata> {
    let mut seen = HashSet::new();
    let mut games: Vec<GameMetadata> = sets
        .into_iter()
        .flatten()
        .filter(|game| seen.insert(game.app_id))
        .collect();
    games.sort_by_key(|game| game.name.to_lowercase());
    games
}

fn merge_play_stats<'a>(
    sources: impl Iterator<Item = &'a HashMap<u32, PlayStats>>,
) -> HashMap<u32, PlayStats> {
    let mut merged: HashMap<u32, PlayStats> = HashMap::new();
    for (app_id, stats) in sources.flat_map(|source| source.iter()) {
        let entry = merged.entry(*app_id).or_default();
        entry.playtime_minutes = entry.playtime_minutes.max(stats.playtime_minutes);
        entry.last_played_unix = entry.last_played_unix.max(stats.last_played_unix);
    }
    merged
}
