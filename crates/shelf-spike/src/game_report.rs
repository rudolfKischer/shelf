use std::collections::{BTreeMap, HashMap};

use shelf_core::collections::Collection;
use shelf_core::highball::WineVerdict;
use shelf_core::launch_plan::{launch_url, LaunchAction};
use shelf_core::library::LibrarySnapshot;
use shelf_core::model::Game;

use crate::report::runtime_label;

const EXAMPLES_PER_CATEGORY: usize = 3;
const MISSING_TOKEN_PLACEHOLDER: &str = "<token>";

pub struct GameReportContext<'a> {
    pub snapshot: &'a LibrarySnapshot,
    pub verdicts: &'a HashMap<u32, WineVerdict>,
    pub config: &'a shelf_core::config::Config,
    pub token: Option<&'a str>,
}

pub fn print_categories(games: &[Game]) {
    let mut by_category: BTreeMap<u32, Vec<&Game>> = BTreeMap::new();
    for game in games {
        for id in &game.metadata.category_ids {
            by_category.entry(*id).or_default().push(game);
        }
    }
    for (id, members) in by_category {
        let examples: Vec<&str> = members
            .iter()
            .take(EXAMPLES_PER_CATEGORY)
            .map(|g| g.metadata.name.as_str())
            .collect();
        println!(
            "category {:>3}  games {:>3}  e.g. {}",
            id,
            members.len(),
            examples.join(" | ")
        );
    }
}

pub fn print_collection(context: &GameReportContext, games: &[Game], needle: &str) {
    let lowered = needle.to_lowercase();
    let Some(collection) = context
        .snapshot
        .collections
        .iter()
        .find(|c| c.name.to_lowercase().contains(&lowered))
    else {
        println!("no collection matches '{needle}'");
        return;
    };
    println!("{} ({} games)", collection.name, collection.app_ids.len());
    for game in games
        .iter()
        .filter(|g| g.collection_ids.contains(&collection.id))
    {
        println!(
            "  {:<40} installed: {}",
            game.metadata.name,
            installed_label(game)
        );
    }
}

pub fn print_games_matching(context: &GameReportContext, games: &[Game], needle: &str) {
    let lowered = needle.to_lowercase();
    for game in games
        .iter()
        .filter(|g| g.metadata.name.to_lowercase().contains(&lowered))
    {
        print_game(context, game);
    }
}

fn installed_label(game: &Game) -> String {
    if game.copies.is_empty() {
        return "no".to_string();
    }
    game.copies
        .iter()
        .map(|copy| runtime_label(&copy.runtime))
        .collect::<Vec<_>>()
        .join("+")
}

fn collection_names<'a>(game: &Game, collections: &'a [Collection]) -> Vec<&'a str> {
    collections
        .iter()
        .filter(|c| game.collection_ids.contains(&c.id))
        .map(|c| c.name.as_str())
        .collect()
}

fn print_game(context: &GameReportContext, game: &Game) {
    let metadata = &game.metadata;
    println!("{} ({})", metadata.name, metadata.app_id);
    println!(
        "  platforms: {:?}  input: {:?}",
        game.features.platforms, game.features.input
    );
    println!(
        "  installed: {}  playtime_min: {}",
        installed_label(game),
        game.play.playtime_minutes
    );
    println!(
        "  collections: {:?}",
        collection_names(game, &context.snapshot.collections)
    );
    println!(
        "  wine verdict: {:?}",
        context
            .verdicts
            .get(&metadata.app_id)
            .map(|v| (&v.status, &v.renderer))
    );
    for action in [LaunchAction::Play, LaunchAction::Install] {
        let url = launch_url(game, action, context.token, context.config);
        println!("  {:?} url: {}", action, redact(url, context.token));
    }
}

fn redact(url: Option<String>, token: Option<&str>) -> String {
    match (url, token) {
        (Some(url), Some(token)) => url.replace(token, MISSING_TOKEN_PLACEHOLDER),
        (Some(url), None) => url,
        (None, _) => "none".to_string(),
    }
}

pub fn print_catalog_stats(config: &shelf_core::config::Config) {
    let started = std::time::Instant::now();
    let catalog = shelf_core::catalog::build_catalog(config);
    let elapsed = started.elapsed();
    let json = serde_json::to_string(&catalog).unwrap_or_default();
    let local_art = catalog
        .games
        .iter()
        .filter(|g| g.art.capsule.local.is_some())
        .count();
    println!(
        "games: {}  collections: {}  with local capsule: {}  without: {}",
        catalog.games.len(),
        catalog.collections.len(),
        local_art,
        catalog.games.len() - local_art
    );
    println!(
        "catalog built in {:?}, json {} KB, runtimes {:?}",
        elapsed,
        json.len() / 1024,
        catalog.runtimes
    );
    if let Some(sample) = catalog.games.iter().find(|g| g.name.contains("Celeste")) {
        println!(
            "sample: {}",
            serde_json::to_string(sample).unwrap_or_default()
        );
    }
}
