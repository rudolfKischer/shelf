use crate::config::Config;
use crate::model::Game;
use crate::steam_install::Runtime;
use crate::templates::fill;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LaunchAction {
    Play,
    Install,
}

pub fn launch_url(
    game: &Game,
    action: LaunchAction,
    highball_token: Option<&str>,
    config: &Config,
) -> Option<String> {
    let app_id = game.metadata.app_id.to_string();
    match action {
        LaunchAction::Play => play_url(game, &app_id, highball_token, config),
        LaunchAction::Install => install_url(game, &app_id, highball_token, config),
    }
}

fn play_url(game: &Game, app_id: &str, token: Option<&str>, config: &Config) -> Option<String> {
    let has_copy_on =
        |wanted: fn(&Runtime) -> bool| game.copies.iter().any(|copy| wanted(&copy.runtime));
    if has_copy_on(is_mac) {
        return Some(fill(&config.steam_run_url_template, &[("app_id", app_id)]));
    }
    if has_copy_on(is_wine) {
        return Some(fill(
            &config.highball_play_url_template,
            &[("app_id", app_id), ("token", token?)],
        ));
    }
    None
}

fn install_url(game: &Game, app_id: &str, token: Option<&str>, config: &Config) -> Option<String> {
    if game.features.platforms.mac {
        return Some(fill(
            &config.steam_install_url_template,
            &[("app_id", app_id)],
        ));
    }
    if game.features.platforms.windows {
        return Some(fill(
            &config.highball_open_steam_url_template,
            &[("token", token?)],
        ));
    }
    None
}

fn is_mac(runtime: &Runtime) -> bool {
    *runtime == Runtime::MacNative
}

fn is_wine(runtime: &Runtime) -> bool {
    matches!(runtime, Runtime::WineBottle(_))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::app_info::GameMetadata;
    use crate::features::{GameFeatures, InputSupport, Platforms};
    use crate::local_config::PlayStats;
    use crate::model::InstalledCopy;

    fn game_with(runtime: Option<Runtime>, mac: bool) -> Game {
        Game {
            metadata: GameMetadata {
                app_id: 814380,
                name: "Sekiro".into(),
                kind: "game".into(),
                os_list: vec![],
                controller_support: None,
                category_ids: vec![],
                store_tag_ids: vec![],
                genre_ids: vec![],
                review_percentage: None,
                metacritic_score: None,
                release_unix: None,
                deck_category: None,
            },
            features: GameFeatures {
                platforms: Platforms {
                    mac,
                    linux: false,
                    windows: true,
                },
                input: InputSupport::ControllerFull,
                single_player: true,
                multiplayer: false,
                local_multiplayer: false,
                online_multiplayer: false,
            },
            copies: runtime
                .map(|runtime| InstalledCopy {
                    runtime,
                    size_on_disk: 0,
                    state_flags: 4,
                })
                .into_iter()
                .collect(),
            collection_ids: vec![],
            hidden: false,
            play: PlayStats::default(),
        }
    }

    #[test]
    fn wine_copy_launches_through_highball_with_token() {
        let game = game_with(Some(Runtime::WineBottle("Games".into())), false);
        let url = launch_url(
            &game,
            LaunchAction::Play,
            Some("abc"),
            &Config::load_default(),
        );
        assert_eq!(url.as_deref(), Some("highball://play/steam/814380?t=abc"));
    }

    #[test]
    fn mac_copy_launches_through_steam() {
        let game = game_with(Some(Runtime::MacNative), true);
        let url = launch_url(&game, LaunchAction::Play, None, &Config::load_default());
        assert_eq!(url.as_deref(), Some("steam://run/814380"));
    }

    #[test]
    fn wine_launch_without_token_is_refused() {
        let game = game_with(Some(Runtime::WineBottle("Games".into())), false);
        assert_eq!(
            launch_url(&game, LaunchAction::Play, None, &Config::load_default()),
            None
        );
    }

    #[test]
    fn windows_only_install_goes_to_highball_steam() {
        let game = game_with(None, false);
        let url = launch_url(
            &game,
            LaunchAction::Install,
            Some("abc"),
            &Config::load_default(),
        );
        assert_eq!(url.as_deref(), Some("highball://open/launcher/steam?t=abc"));
    }
}
