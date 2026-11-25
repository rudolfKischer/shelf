mod game_report;
mod report;

use shelf_core::config::Config;
use shelf_core::features::CategoryFeatureIds;
use shelf_core::highball::{find_highball_app, load_wine_verdicts, read_launch_token};
use shelf_core::library::load_snapshot;
use shelf_core::model::build_games;
use shelf_core::steam_install::home_dir;

use game_report::GameReportContext;

const COMMAND_CATEGORIES: &str = "categories";
const COMMAND_GAME: &str = "game";
const COMMAND_COLLECTION: &str = "collection";
const COMMAND_CATALOG: &str = "catalog";

fn main() {
    let config = Config::load_default();
    let home = home_dir().unwrap_or_default();
    let snapshot = load_snapshot(&config);
    let games = build_games(&snapshot, &CategoryFeatureIds::load_default(), &config);
    let verdicts = find_highball_app(&home, &config)
        .map(|app| load_wine_verdicts(&app, &config))
        .unwrap_or_default();
    let token = read_launch_token(&home, &config);
    let context = GameReportContext {
        snapshot: &snapshot,
        verdicts: &verdicts,
        config: &config,
        token: token.as_deref(),
    };
    let arguments: Vec<String> = std::env::args().skip(1).collect();
    let rest = arguments
        .iter()
        .skip(1)
        .cloned()
        .collect::<Vec<_>>()
        .join(" ");
    match arguments.first().map(String::as_str) {
        Some(COMMAND_CATEGORIES) => game_report::print_categories(&games),
        Some(COMMAND_GAME) => game_report::print_games_matching(&context, &games, &rest),
        Some(COMMAND_CATALOG) => game_report::print_catalog_stats(&config),
        Some(COMMAND_COLLECTION) => game_report::print_collection(&context, &games, &rest),
        _ => report::print_summary(&snapshot, &games, &verdicts),
    }
}
