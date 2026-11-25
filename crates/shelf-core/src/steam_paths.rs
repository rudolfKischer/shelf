use std::fs;
use std::path::{Path, PathBuf};
use std::time::SystemTime;

use crate::config::Config;

#[derive(Debug)]
pub struct SteamPaths {
    pub library_cache: PathBuf,
    pub app_info: PathBuf,
    pub steamapps: PathBuf,
    pub library_folders: PathBuf,
    pub local_config: Option<PathBuf>,
    pub collections: Option<PathBuf>,
}

pub fn locate(root: &Path, config: &Config) -> SteamPaths {
    let user_config = find_user_config_dir(&root.join(&config.user_data_dir), config);
    let steamapps = root.join(&config.steamapps_dir);
    SteamPaths {
        library_cache: root.join(&config.library_cache_dir),
        app_info: root.join(&config.app_info_file),
        library_folders: steamapps.join(&config.library_folders_file),
        local_config: user_config
            .as_ref()
            .map(|dir| dir.join(&config.local_config_file)),
        collections: user_config.map(|dir| dir.join(&config.collections_file)),
        steamapps,
    }
}

fn find_user_config_dir(user_data: &Path, config: &Config) -> Option<PathBuf> {
    let entries = fs::read_dir(user_data).ok()?;
    entries
        .filter_map(Result::ok)
        .filter(|entry| is_real_user_id(&entry.file_name().to_string_lossy()))
        .map(|entry| entry.path().join(&config.user_config_dir))
        .filter_map(|dir| {
            last_modified(&dir.join(&config.local_config_file)).map(|time| (time, dir))
        })
        .max_by_key(|(time, _)| *time)
        .map(|(_, dir)| dir)
}

fn is_real_user_id(directory_name: &str) -> bool {
    directory_name.parse::<u64>().map_or(false, |id| id > 0)
}

fn last_modified(path: &Path) -> Option<SystemTime> {
    fs::metadata(path).and_then(|meta| meta.modified()).ok()
}
