use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

use crate::app_info::{metadata_of, GameMetadata};
use crate::collections::{read_collections, Collection};
use crate::config::Config;
use crate::local_config::{read_play_stats, PlayStats};
use crate::manifests::{read_installed_games, steamapps_directories, InstalledGame};
use crate::steam_install::SteamInstall;
use crate::steam_paths::{self, SteamPaths};
use crate::vdf_binary::{read_app_info, AppInfoError};

#[derive(Debug)]
pub enum LibraryError {
    ReadAppInfo(String),
    ParseAppInfo(AppInfoError),
}

#[derive(Debug)]
pub struct InstallSnapshot {
    pub install: SteamInstall,
    pub games: Vec<GameMetadata>,
    pub installed: Vec<InstalledGame>,
    pub collections: Vec<Collection>,
    pub play_stats: HashMap<u32, PlayStats>,
}

pub fn load_install(
    install: &SteamInstall,
    config: &Config,
) -> Result<InstallSnapshot, LibraryError> {
    let paths = steam_paths::locate(&install.root, config);
    let owned = owned_app_ids(&paths.library_cache);
    Ok(InstallSnapshot {
        install: install.clone(),
        games: load_games(&paths, &owned, config)?,
        installed: read_installed_games(&steamapps_directories(&paths, config), config),
        collections: optional_file(&paths.collections, |file| read_collections(file, config)),
        play_stats: optional_file(&paths.local_config, |file| read_play_stats(file, config)),
    })
}

pub fn owned_app_ids(library_cache: &Path) -> HashSet<u32> {
    let Ok(entries) = fs::read_dir(library_cache) else {
        return HashSet::new();
    };
    entries
        .filter_map(Result::ok)
        .filter_map(|entry| entry.file_name().to_string_lossy().parse().ok())
        .collect()
}

fn load_games(
    paths: &SteamPaths,
    owned: &HashSet<u32>,
    config: &Config,
) -> Result<Vec<GameMetadata>, LibraryError> {
    let bytes =
        fs::read(&paths.app_info).map_err(|error| LibraryError::ReadAppInfo(error.to_string()))?;
    let entries = read_app_info(&bytes, Some(owned)).map_err(LibraryError::ParseAppInfo)?;
    Ok(entries
        .iter()
        .filter_map(|entry| metadata_of(entry, config))
        .filter(|game| game.kind.eq_ignore_ascii_case(&config.game_type))
        .collect())
}

fn optional_file<T: Default>(file: &Option<std::path::PathBuf>, read: impl Fn(&Path) -> T) -> T {
    file.as_deref().map(read).unwrap_or_default()
}
