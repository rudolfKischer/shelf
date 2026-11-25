use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::config::Config;

const FAVORITES_ID: &str = "favorite";
const HIDDEN_ID: &str = "hidden";

#[derive(Debug, Clone, PartialEq)]
pub enum CollectionKind {
    Custom,
    Favorites,
    Hidden,
    Dynamic,
}

#[derive(Debug, Clone)]
pub struct Collection {
    pub id: String,
    pub name: String,
    pub kind: CollectionKind,
    pub app_ids: Vec<u32>,
    pub version: u64,
    pub timestamp: u64,
    pub deleted: bool,
}

#[derive(Deserialize)]
struct StorageEntry(String, StorageRecord);

#[derive(Deserialize)]
struct StorageRecord {
    value: Option<String>,
    #[serde(default)]
    is_deleted: bool,
    version: Option<String>,
    timestamp: Option<u64>,
}

#[derive(Deserialize)]
struct CollectionValue {
    name: String,
    #[serde(default)]
    added: Vec<u32>,
    #[serde(rename = "filterSpec")]
    filter_spec: Option<serde_json::Value>,
}

pub fn read_collections(file: &Path, config: &Config) -> Vec<Collection> {
    let Ok(text) = fs::read_to_string(file) else {
        return Vec::new();
    };
    parse_collections(&text, &config.collection_key_prefix)
}

pub fn parse_collections(text: &str, key_prefix: &str) -> Vec<Collection> {
    let Ok(entries) = serde_json::from_str::<Vec<StorageEntry>>(text) else {
        return Vec::new();
    };
    entries
        .into_iter()
        .filter_map(|StorageEntry(key, record)| {
            let id = key.strip_prefix(key_prefix)?.to_string();
            Some(into_collection(id, record))
        })
        .collect()
}

fn into_collection(id: String, record: StorageRecord) -> Collection {
    let version = record
        .version
        .as_deref()
        .and_then(|v| v.parse().ok())
        .unwrap_or(0);
    let timestamp = record.timestamp.unwrap_or(0);
    let value = record
        .value
        .as_deref()
        .filter(|_| !record.is_deleted)
        .and_then(|text| serde_json::from_str::<CollectionValue>(text).ok());
    match value {
        Some(value) => Collection {
            kind: kind_of(&id, &value),
            name: value.name,
            app_ids: value.added,
            id,
            version,
            timestamp,
            deleted: false,
        },
        None => Collection {
            id,
            name: String::new(),
            kind: CollectionKind::Custom,
            app_ids: Vec::new(),
            version,
            timestamp,
            deleted: true,
        },
    }
}

fn kind_of(id: &str, value: &CollectionValue) -> CollectionKind {
    match id {
        _ if value.filter_spec.is_some() => CollectionKind::Dynamic,
        FAVORITES_ID => CollectionKind::Favorites,
        HIDDEN_ID => CollectionKind::Hidden,
        _ => CollectionKind::Custom,
    }
}

pub fn merge_newest(sources: Vec<Vec<Collection>>) -> Vec<Collection> {
    let mut order: Vec<String> = Vec::new();
    let mut newest: HashMap<String, Collection> = HashMap::new();
    for collection in sources.into_iter().flatten() {
        if !newest.contains_key(&collection.id) {
            order.push(collection.id.clone());
        }
        let is_newer = newest.get(&collection.id).map_or(true, |current| {
            (collection.version, collection.timestamp) > (current.version, current.timestamp)
        });
        if is_newer {
            newest.insert(collection.id.clone(), collection);
        }
    }
    order
        .into_iter()
        .filter_map(|id| newest.remove(&id))
        .filter(|collection| !collection.deleted)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const CURRENT: &str = include_str!("../tests/fixtures/sample_collections.json");
    const STALE: &str = include_str!("../tests/fixtures/stale_collections.json");
    const PREFIX: &str = "user-collections.";

    #[test]
    fn parses_live_collections_and_tombstones() {
        let collections = parse_collections(CURRENT, PREFIX);
        let live: Vec<_> = collections
            .iter()
            .filter(|c| !c.deleted)
            .map(|c| c.name.as_str())
            .collect();
        assert_eq!(live, ["Favorites", "GEMS"]);
        assert!(collections.iter().any(|c| c.deleted));
    }

    #[test]
    fn newest_version_wins_over_stale_copy() {
        let merged = merge_newest(vec![
            parse_collections(STALE, PREFIX),
            parse_collections(CURRENT, PREFIX),
        ]);
        let gems = merged.iter().find(|c| c.name == "GEMS").unwrap();
        assert_eq!(gems.app_ids, [413150, 504230]);
    }

    #[test]
    fn newer_deletion_beats_stale_live_copy() {
        let merged = merge_newest(vec![
            parse_collections(STALE, PREFIX),
            parse_collections(CURRENT, PREFIX),
        ]);
        assert!(merged.iter().all(|c| c.id != "uc-deleted"));
    }

    #[test]
    fn game_can_belong_to_several_collections() {
        let merged = merge_newest(vec![parse_collections(CURRENT, PREFIX)]);
        let containing: Vec<_> = merged
            .iter()
            .filter(|c| c.app_ids.contains(&504230))
            .collect();
        assert_eq!(containing.len(), 2);
    }
}
