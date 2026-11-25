use std::collections::{BTreeMap, HashMap};

use shelf_core::collections::CollectionKind;
use shelf_core::highball::WineVerdict;
use shelf_core::library::LibrarySnapshot;
use shelf_core::model::Game;
use shelf_core::steam_install::Runtime;

pub fn runtime_label(runtime: &Runtime) -> String {
    match runtime {
        Runtime::MacNative => "mac".to_string(),
        Runtime::WineBottle(name) => format!("wine:{name}"),
    }
}

pub fn print_summary(
    snapshot: &LibrarySnapshot,
    games: &[Game],
    verdicts: &HashMap<u32, WineVerdict>,
) {
    println!(
        "steam installs read: {}",
        snapshot
            .runtimes
            .iter()
            .map(runtime_label)
            .collect::<Vec<_>>()
            .join(", ")
    );
    for problem in &snapshot.problems {
        println!(
            "problem in {}: {:?}",
            runtime_label(&problem.runtime),
            problem.error
        );
    }
    println!("owned games: {}", games.len());
    print_install_counts(games);
    print_collections(snapshot, games);
    print_windows_only(games, verdicts);
}

fn print_install_counts(games: &[Game]) {
    let mut by_runtime: BTreeMap<String, usize> = BTreeMap::new();
    for copy in games.iter().flat_map(|game| game.copies.iter()) {
        *by_runtime.entry(runtime_label(&copy.runtime)).or_default() += 1;
    }
    let on_both = games.iter().filter(|game| game.copies.len() > 1).count();
    println!(
        "installed copies: {:?}  (installed in more than one place: {})",
        by_runtime, on_both
    );
}

fn print_collections(snapshot: &LibrarySnapshot, games: &[Game]) {
    println!("collections:");
    for collection in &snapshot.collections {
        println!(
            "  {:<40} {:>4}  {}",
            collection.name,
            collection.app_ids.len(),
            kind_label(&collection.kind)
        );
    }
    let mut by_membership: BTreeMap<usize, usize> = BTreeMap::new();
    for game in games {
        *by_membership.entry(game.collection_ids.len()).or_default() += 1;
    }
    println!(
        "games by number of collections they are in: {:?}",
        by_membership
    );
}

fn kind_label(kind: &CollectionKind) -> &'static str {
    match kind {
        CollectionKind::Custom => "custom",
        CollectionKind::Favorites => "favorites",
        CollectionKind::Hidden => "hidden",
        CollectionKind::Dynamic => "dynamic",
    }
}

fn print_windows_only(games: &[Game], verdicts: &HashMap<u32, WineVerdict>) {
    let windows_only: Vec<&Game> = games.iter().filter(|g| !g.features.platforms.mac).collect();
    let mut by_status: BTreeMap<String, usize> = BTreeMap::new();
    for game in &windows_only {
        let status = verdicts
            .get(&game.metadata.app_id)
            .map_or("no verdict".to_string(), |verdict| verdict.status.clone());
        *by_status.entry(status).or_default() += 1;
    }
    println!(
        "windows-only games: {}  wine verdicts: {:?}",
        windows_only.len(),
        by_status
    );
}
