use serde::{Deserialize, Serialize};

use crate::app_info::GameMetadata;

const CATEGORY_FEATURES_JSON: &str = include_str!("../config/category_features.json");
const OS_MAC: &str = "macos";
const OS_LINUX: &str = "linux";
const OS_WINDOWS: &str = "windows";
const CONTROLLER_SUPPORT_FULL: &str = "full";
const CONTROLLER_SUPPORT_PARTIAL: &str = "partial";

#[derive(Debug, Clone, Deserialize)]
pub struct CategoryFeatureIds {
    pub single_player: Vec<u32>,
    pub multiplayer: Vec<u32>,
    pub coop: Vec<u32>,
    pub online_coop: Vec<u32>,
    pub local_coop: Vec<u32>,
    pub shared_screen: Vec<u32>,
    pub local_pvp: Vec<u32>,
    pub online_pvp: Vec<u32>,
    pub cross_platform: Vec<u32>,
    pub remote_play_together: Vec<u32>,
    pub full_controller: Vec<u32>,
    pub partial_controller: Vec<u32>,
}

impl CategoryFeatureIds {
    pub fn load_default() -> CategoryFeatureIds {
        serde_json::from_str(CATEGORY_FEATURES_JSON)
            .expect("embedded category features must be valid")
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub enum InputSupport {
    ControllerFull,
    ControllerPartial,
    KeyboardMouseOnly,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize)]
pub struct Platforms {
    pub mac: bool,
    pub linux: bool,
    pub windows: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct GameFeatures {
    pub platforms: Platforms,
    pub input: InputSupport,
    pub single_player: bool,
    pub multiplayer: bool,
    pub local_multiplayer: bool,
    pub online_multiplayer: bool,
}

pub fn features_of(game: &GameMetadata, ids: &CategoryFeatureIds) -> GameFeatures {
    let has = |wanted: &[u32]| game.category_ids.iter().any(|id| wanted.contains(id));
    GameFeatures {
        platforms: platforms_of(&game.os_list),
        input: input_support_of(game, ids),
        single_player: has(&ids.single_player),
        multiplayer: has(&ids.multiplayer),
        local_multiplayer: has(&ids.local_coop) || has(&ids.shared_screen) || has(&ids.local_pvp),
        online_multiplayer: has(&ids.online_coop) || has(&ids.online_pvp),
    }
}

fn platforms_of(os_list: &[String]) -> Platforms {
    let lists = |name: &str| os_list.iter().any(|os| os == name);
    Platforms {
        mac: lists(OS_MAC),
        linux: lists(OS_LINUX),
        windows: lists(OS_WINDOWS),
    }
}

fn input_support_of(game: &GameMetadata, ids: &CategoryFeatureIds) -> InputSupport {
    let has = |wanted: &[u32]| game.category_ids.iter().any(|id| wanted.contains(id));
    let declared = game.controller_support.as_deref();
    if declared == Some(CONTROLLER_SUPPORT_FULL) || has(&ids.full_controller) {
        return InputSupport::ControllerFull;
    }
    if declared == Some(CONTROLLER_SUPPORT_PARTIAL) || has(&ids.partial_controller) {
        return InputSupport::ControllerPartial;
    }
    InputSupport::KeyboardMouseOnly
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_game(category_ids: Vec<u32>, os: &[&str]) -> GameMetadata {
        GameMetadata {
            app_id: 1,
            name: "Sample".into(),
            kind: "game".into(),
            os_list: os.iter().map(|name| name.to_string()).collect(),
            controller_support: None,
            category_ids,
            store_tag_ids: vec![],
            genre_ids: vec![],
            review_percentage: None,
            metacritic_score: None,
            release_unix: None,
            deck_category: None,
        }
    }

    #[test]
    fn stardew_like_game_has_local_and_online_multiplayer() {
        let ids = CategoryFeatureIds::load_default();
        let features = features_of(
            &sample_game(vec![2, 1, 9, 38, 39, 24], &["windows", "macos"]),
            &ids,
        );
        assert!(features.local_multiplayer && features.online_multiplayer);
        assert!(features.platforms.mac && !features.platforms.linux);
    }

    #[test]
    fn game_without_controller_flags_is_keyboard_only() {
        let ids = CategoryFeatureIds::load_default();
        let features = features_of(&sample_game(vec![2], &["windows"]), &ids);
        assert_eq!(features.input, InputSupport::KeyboardMouseOnly);
        assert!(!features.platforms.mac);
    }
}
