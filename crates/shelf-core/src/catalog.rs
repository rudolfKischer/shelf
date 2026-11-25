use std::collections::HashMap;

use serde::Serialize;

use crate::art::{resolve_art, ArtSet};
use crate::collections::{Collection, CollectionKind};
use crate::config::Config;
use crate::features::{CategoryFeatureIds, GameFeatures};
use crate::highball::{find_highball_app, load_wine_verdicts, WineVerdict};
use crate::library::{load_snapshot, LibrarySnapshot};
use crate::model::{build_games, Game, InstalledCopy};
use crate::steam_install::{home_dir, Runtime};

const RUNTIME_MAC: &str = "mac";
const RUNTIME_WINE: &str = "wine";
const KIND_CUSTOM: &str = "custom";
const KIND_FAVORITES: &str = "favorites";
const KIND_HIDDEN: &str = "hidden";
const KIND_DYNAMIC: &str = "dynamic";

#[derive(Debug, Serialize)]
pub struct CatalogCopy {
    pub runtime: &'static str,
    pub bottle: Option<String>,
    pub size_on_disk: u64,
}

#[derive(Debug, Serialize)]
pub struct CatalogWine {
    pub status: String,
    pub renderer: Option<String>,
    pub blocked: bool,
}

#[derive(Debug, Serialize)]
pub struct CatalogEntry {
    pub app_id: u32,
    pub name: String,
    pub art: ArtSet,
    pub features: GameFeatures,
    pub copies: Vec<CatalogCopy>,
    pub collection_ids: Vec<String>,
    pub hidden: bool,
    pub playtime_minutes: u64,
    pub last_played_unix: u64,
    pub review_percentage: Option<u32>,
    pub metacritic_score: Option<u32>,
    pub release_unix: Option<u64>,
    pub wine: Option<CatalogWine>,
}

#[derive(Debug, Serialize)]
pub struct CatalogCollection {
    pub id: String,
    pub name: String,
    pub kind: &'static str,
    pub app_ids: Vec<u32>,
}

#[derive(Debug, Serialize)]
pub struct Catalog {
    pub games: Vec<CatalogEntry>,
    pub collections: Vec<CatalogCollection>,
    pub runtimes: Vec<&'static str>,
    pub problems: Vec<String>,
}

pub fn build_catalog(config: &Config) -> Catalog {
    let snapshot = load_snapshot(config);
    let verdicts = load_verdicts(config);
    let games = build_games(&snapshot, &CategoryFeatureIds::load_default(), config);
    Catalog {
        games: games
            .iter()
            .map(|game| entry_of(game, &snapshot, &verdicts, config))
            .collect(),
        collections: snapshot.collections.iter().map(collection_of).collect(),
        runtimes: snapshot.runtimes.iter().map(runtime_label).collect(),
        problems: snapshot
            .problems
            .iter()
            .map(|p| format!("{:?}: {:?}", p.runtime, p.error))
            .collect(),
    }
}

fn load_verdicts(config: &Config) -> HashMap<u32, WineVerdict> {
    let Some(home) = home_dir() else {
        return HashMap::new();
    };
    find_highball_app(&home, config)
        .map(|app| load_wine_verdicts(&app, config))
        .unwrap_or_default()
}

fn entry_of(
    game: &Game,
    snapshot: &LibrarySnapshot,
    verdicts: &HashMap<u32, WineVerdict>,
    config: &Config,
) -> CatalogEntry {
    let metadata = &game.metadata;
    CatalogEntry {
        app_id: metadata.app_id,
        name: metadata.name.clone(),
        art: resolve_art(&snapshot.library_caches, metadata.app_id, config),
        features: game.features.clone(),
        copies: game.copies.iter().map(copy_of).collect(),
        collection_ids: game.collection_ids.clone(),
        hidden: game.hidden,
        playtime_minutes: game.play.playtime_minutes,
        last_played_unix: game.play.last_played_unix,
        review_percentage: metadata.review_percentage,
        metacritic_score: metadata.metacritic_score,
        release_unix: metadata.release_unix,
        wine: verdicts.get(&metadata.app_id).map(wine_of),
    }
}

fn copy_of(copy: &InstalledCopy) -> CatalogCopy {
    let bottle = match &copy.runtime {
        Runtime::WineBottle(name) => Some(name.clone()),
        Runtime::MacNative => None,
    };
    CatalogCopy {
        runtime: runtime_label(&copy.runtime),
        bottle,
        size_on_disk: copy.size_on_disk,
    }
}

fn wine_of(verdict: &WineVerdict) -> CatalogWine {
    CatalogWine {
        status: verdict.status.clone(),
        renderer: verdict.renderer.clone(),
        blocked: verdict.is_blocked(),
    }
}

fn collection_of(collection: &Collection) -> CatalogCollection {
    let kind = match collection.kind {
        CollectionKind::Custom => KIND_CUSTOM,
        CollectionKind::Favorites => KIND_FAVORITES,
        CollectionKind::Hidden => KIND_HIDDEN,
        CollectionKind::Dynamic => KIND_DYNAMIC,
    };
    CatalogCollection {
        id: collection.id.clone(),
        name: collection.name.clone(),
        kind,
        app_ids: collection.app_ids.clone(),
    }
}

fn runtime_label(runtime: &Runtime) -> &'static str {
    match runtime {
        Runtime::MacNative => RUNTIME_MAC,
        Runtime::WineBottle(_) => RUNTIME_WINE,
    }
}
