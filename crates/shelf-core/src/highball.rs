use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use serde::Deserialize;

use crate::config::Config;

const JSON_EXTENSION: &str = "json";
const STATUS_BLOCKED_PREFIX: &str = "blocked";

#[derive(Debug, Clone, Deserialize)]
pub struct AntiCheat {
    #[serde(default)]
    pub names: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct WineVerdict {
    pub steam_appid: Option<u32>,
    pub status: String,
    pub renderer: Option<String>,
    pub anticheat: Option<AntiCheat>,
}

impl WineVerdict {
    pub fn is_blocked(&self) -> bool {
        self.status.starts_with(STATUS_BLOCKED_PREFIX)
    }
}

pub fn read_launch_token(home: &Path, config: &Config) -> Option<String> {
    let file = home
        .join(&config.highball_data_relative_to_home)
        .join(&config.highball_launch_token_file);
    let token = fs::read_to_string(file).ok()?.trim().to_string();
    (!token.is_empty()).then_some(token)
}

pub fn find_highball_app(home: &Path, config: &Config) -> Option<PathBuf> {
    config
        .highball_app_candidates
        .iter()
        .map(|candidate| home.join(candidate))
        .find(|path| path.is_dir())
}

pub fn load_wine_verdicts(app: &Path, config: &Config) -> HashMap<u32, WineVerdict> {
    let directory = app.join(&config.highball_db_relative_to_app);
    let Ok(entries) = fs::read_dir(directory) else {
        return HashMap::new();
    };
    entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().map_or(false, |ext| ext == JSON_EXTENSION))
        .filter_map(|path| fs::read_to_string(path).ok())
        .filter_map(|text| serde_json::from_str::<WineVerdict>(&text).ok())
        .filter_map(|verdict| Some((verdict.steam_appid?, verdict)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocked_statuses_are_detected() {
        let text = r#"{"steam_appid": 5, "status": "blocked-anticheat", "renderer": null, "anticheat": {"names": ["Vanguard"]}}"#;
        let verdict: WineVerdict = serde_json::from_str(text).unwrap();
        assert!(verdict.is_blocked());
        assert_eq!(verdict.anticheat.unwrap().names, ["Vanguard"]);
    }
}
