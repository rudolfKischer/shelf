use std::fs;
use std::path::{Path, PathBuf};

use crate::config::Config;
use crate::steam_paths::SteamPaths;
use crate::vdf_text::{self, Vdf};

const KEY_APP_STATE: &str = "AppState";
const KEY_APP_ID: &str = "appid";
const KEY_NAME: &str = "name";
const KEY_INSTALL_DIR: &str = "installdir";
const KEY_SIZE_ON_DISK: &str = "SizeOnDisk";
const KEY_LAST_PLAYED: &str = "LastPlayed";
const KEY_STATE_FLAGS: &str = "StateFlags";
const KEY_LIBRARY_FOLDERS: &str = "libraryfolders";
const KEY_LIBRARY_PATH: &str = "path";

#[derive(Debug, Clone)]
pub struct InstalledGame {
    pub app_id: u32,
    pub name: String,
    pub install_dir: String,
    pub size_on_disk: u64,
    pub last_played_unix: u64,
    pub state_flags: u32,
}

impl InstalledGame {
    pub fn is_fully_installed(&self, fully_installed_flag: u32) -> bool {
        self.state_flags & fully_installed_flag != 0
    }
}

pub fn steamapps_directories(paths: &SteamPaths, config: &Config) -> Vec<PathBuf> {
    let mut directories = vec![paths.steamapps.clone()];
    let extra = fs::read_to_string(&paths.library_folders)
        .ok()
        .and_then(|text| vdf_text::parse(&text).ok())
        .map(|root| library_paths(&root))
        .unwrap_or_default();
    for library in extra {
        let steamapps = library.join(&config.steamapps_dir);
        if !directories.contains(&steamapps) {
            directories.push(steamapps);
        }
    }
    directories
}

fn library_paths(root: &Vdf) -> Vec<PathBuf> {
    let Some(folders) = root.get(KEY_LIBRARY_FOLDERS) else {
        return Vec::new();
    };
    folders
        .entries()
        .iter()
        .filter_map(|(_, folder)| folder.text_at(&[KEY_LIBRARY_PATH]))
        .map(PathBuf::from)
        .collect()
}

pub fn read_installed_games(directories: &[PathBuf], config: &Config) -> Vec<InstalledGame> {
    directories
        .iter()
        .flat_map(|directory| manifest_files(directory, config))
        .filter_map(|file| fs::read_to_string(file).ok())
        .filter_map(|text| parse_manifest(&text))
        .collect()
}

fn manifest_files(directory: &Path, config: &Config) -> Vec<PathBuf> {
    let Ok(entries) = fs::read_dir(directory) else {
        return Vec::new();
    };
    entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| is_manifest(path, config))
        .collect()
}

fn is_manifest(path: &Path, config: &Config) -> bool {
    let name_matches = path.file_name().map_or(false, |name| {
        name.to_string_lossy().starts_with(&config.manifest_prefix)
    });
    let extension_matches = path.extension().map_or(false, |extension| {
        extension.to_string_lossy() == config.manifest_extension
    });
    name_matches && extension_matches
}

pub fn parse_manifest(text: &str) -> Option<InstalledGame> {
    let root = vdf_text::parse(text).ok()?;
    let state = root.get(KEY_APP_STATE)?;
    Some(InstalledGame {
        app_id: state.number_at(&[KEY_APP_ID])? as u32,
        name: state.text_at(&[KEY_NAME])?.to_string(),
        install_dir: state
            .text_at(&[KEY_INSTALL_DIR])
            .unwrap_or_default()
            .to_string(),
        size_on_disk: state.number_at(&[KEY_SIZE_ON_DISK]).unwrap_or(0),
        last_played_unix: state.number_at(&[KEY_LAST_PLAYED]).unwrap_or(0),
        state_flags: state.number_at(&[KEY_STATE_FLAGS]).unwrap_or(0) as u32,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = include_str!("../tests/fixtures/sample_manifest.acf");

    #[test]
    fn parses_manifest_fields() {
        let game = parse_manifest(SAMPLE).unwrap();
        assert_eq!(game.app_id, 1030300);
        assert_eq!(game.size_on_disk, 8260628780);
        assert!(game.is_fully_installed(4));
    }
}
