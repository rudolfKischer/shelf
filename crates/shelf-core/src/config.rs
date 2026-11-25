use serde::Deserialize;

const DEFAULT_CONFIG_JSON: &str = include_str!("../config/default.json");

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub steam_root_relative_to_home: String,
    pub highball_bottles_relative_to_home: String,
    pub wine_steam_relative_to_bottle: String,
    pub highball_data_relative_to_home: String,
    pub highball_launch_token_file: String,
    pub highball_app_candidates: Vec<String>,
    pub highball_db_relative_to_app: String,
    pub steam_run_url_template: String,
    pub steam_install_url_template: String,
    pub highball_play_url_template: String,
    pub highball_open_steam_url_template: String,
    pub library_cache_dir: String,
    pub app_info_file: String,
    pub steamapps_dir: String,
    pub library_folders_file: String,
    pub manifest_prefix: String,
    pub manifest_extension: String,
    pub user_data_dir: String,
    pub user_config_dir: String,
    pub local_config_file: String,
    pub collections_file: String,
    pub collection_key_prefix: String,
    pub local_config_apps_path: Vec<String>,
    pub fully_installed_flag: u32,
    pub game_type: String,
    pub os_list_separator: String,
    pub art_capsule_files: Vec<String>,
    pub art_hero_files: Vec<String>,
    pub art_logo_files: Vec<String>,
    pub art_cdn_capsule_template: String,
    pub art_cdn_hero_template: String,
}

impl Config {
    pub fn load_default() -> Config {
        serde_json::from_str(DEFAULT_CONFIG_JSON).expect("embedded default config must be valid")
    }
}
