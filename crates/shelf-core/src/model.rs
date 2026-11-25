use std::collections::HashMap;

use crate::app_info::GameMetadata;
use crate::collections::{Collection, CollectionKind};
use crate::config::Config;
use crate::features::{features_of, CategoryFeatureIds, GameFeatures};
use crate::library::LibrarySnapshot;
use crate::local_config::PlayStats;
use crate::steam_install::Runtime;

#[derive(Debug, Clone)]
pub struct InstalledCopy {
    pub runtime: Runtime,
    pub size_on_disk: u64,
    pub state_flags: u32,
}

#[derive(Debug, Clone)]
pub struct Game {
    pub metadata: GameMetadata,
    pub features: GameFeatures,
    pub copies: Vec<InstalledCopy>,
    pub collection_ids: Vec<String>,
    pub hidden: bool,
    pub play: PlayStats,
}

pub fn build_games(
    snapshot: &LibrarySnapshot,
    ids: &CategoryFeatureIds,
    config: &Config,
) -> Vec<Game> {
    let memberships = collection_memberships(&snapshot.collections);
    let copies = installed_copies(snapshot, config);
    snapshot
        .games
        .iter()
        .map(|metadata| Game {
            features: features_of(metadata, ids),
            copies: copies.get(&metadata.app_id).cloned().unwrap_or_default(),
            collection_ids: memberships
                .get(&metadata.app_id)
                .cloned()
                .unwrap_or_default(),
            hidden: is_hidden(metadata.app_id, &snapshot.collections),
            play: snapshot
                .play_stats
                .get(&metadata.app_id)
                .cloned()
                .unwrap_or_default(),
            metadata: metadata.clone(),
        })
        .collect()
}

pub fn collection_memberships(collections: &[Collection]) -> HashMap<u32, Vec<String>> {
    let mut memberships: HashMap<u32, Vec<String>> = HashMap::new();
    for collection in collections
        .iter()
        .filter(|c| c.kind != CollectionKind::Hidden)
    {
        for app_id in &collection.app_ids {
            memberships
                .entry(*app_id)
                .or_default()
                .push(collection.id.clone());
        }
    }
    memberships
}

fn is_hidden(app_id: u32, collections: &[Collection]) -> bool {
    collections
        .iter()
        .any(|c| c.kind == CollectionKind::Hidden && c.app_ids.contains(&app_id))
}

fn installed_copies(
    snapshot: &LibrarySnapshot,
    config: &Config,
) -> HashMap<u32, Vec<InstalledCopy>> {
    let mut copies: HashMap<u32, Vec<InstalledCopy>> = HashMap::new();
    for entry in snapshot
        .installed
        .iter()
        .filter(|entry| entry.game.is_fully_installed(config.fully_installed_flag))
    {
        copies
            .entry(entry.game.app_id)
            .or_default()
            .push(InstalledCopy {
                runtime: entry.runtime.clone(),
                size_on_disk: entry.game.size_on_disk,
                state_flags: entry.game.state_flags,
            });
    }
    copies
}
