use std::fs;
use std::path::{Path, PathBuf};

use crate::config::Config;

const HOME_ENV_VAR: &str = "HOME";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Runtime {
    MacNative,
    WineBottle(String),
}

#[derive(Debug, Clone)]
pub struct SteamInstall {
    pub runtime: Runtime,
    pub root: PathBuf,
}

pub fn home_dir() -> Option<PathBuf> {
    std::env::var_os(HOME_ENV_VAR).map(PathBuf::from)
}

pub fn discover_installs(home: &Path, config: &Config) -> Vec<SteamInstall> {
    let mac_root = home.join(&config.steam_root_relative_to_home);
    let mac = mac_root.is_dir().then(|| SteamInstall {
        runtime: Runtime::MacNative,
        root: mac_root,
    });
    mac.into_iter()
        .chain(discover_wine_installs(home, config))
        .collect()
}

fn discover_wine_installs(home: &Path, config: &Config) -> Vec<SteamInstall> {
    let bottles_dir = home.join(&config.highball_bottles_relative_to_home);
    let Ok(entries) = fs::read_dir(bottles_dir) else {
        return Vec::new();
    };
    let mut installs: Vec<SteamInstall> = entries
        .filter_map(Result::ok)
        .filter_map(|entry| wine_install_in(&entry.path(), config))
        .collect();
    installs.sort_by(|a, b| a.root.cmp(&b.root));
    installs
}

fn wine_install_in(bottle: &Path, config: &Config) -> Option<SteamInstall> {
    let root = bottle.join(&config.wine_steam_relative_to_bottle);
    if !root.is_dir() {
        return None;
    }
    let name = bottle.file_name()?.to_string_lossy().into_owned();
    Some(SteamInstall {
        runtime: Runtime::WineBottle(name),
        root,
    })
}
