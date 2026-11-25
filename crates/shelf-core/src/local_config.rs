use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::config::Config;
use crate::vdf_text::{self, Vdf};

const KEY_PLAYTIME: &str = "Playtime";
const KEY_LAST_PLAYED: &str = "LastPlayed";

#[derive(Debug, Clone, Default)]
pub struct PlayStats {
    pub playtime_minutes: u64,
    pub last_played_unix: u64,
}

pub fn read_play_stats(file: &Path, config: &Config) -> HashMap<u32, PlayStats> {
    let Ok(text) = fs::read_to_string(file) else {
        return HashMap::new();
    };
    let Ok(root) = vdf_text::parse(&text) else {
        return HashMap::new();
    };
    let path: Vec<&str> = config
        .local_config_apps_path
        .iter()
        .map(String::as_str)
        .collect();
    root.path(&[&["UserLocalConfigStore"], path.as_slice()].concat())
        .map(play_stats_by_app)
        .unwrap_or_default()
}

fn play_stats_by_app(apps: &Vdf) -> HashMap<u32, PlayStats> {
    apps.entries()
        .iter()
        .filter_map(|(key, app)| Some((key.parse::<u32>().ok()?, stats_of(app)?)))
        .collect()
}

fn stats_of(app: &Vdf) -> Option<PlayStats> {
    let playtime = app.number_at(&[KEY_PLAYTIME]);
    let last_played = app.number_at(&[KEY_LAST_PLAYED]);
    if playtime.is_none() && last_played.is_none() {
        return None;
    }
    Some(PlayStats {
        playtime_minutes: playtime.unwrap_or(0),
        last_played_unix: last_played.unwrap_or(0),
    })
}
