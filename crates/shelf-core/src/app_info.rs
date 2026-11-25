use crate::app_info_keys as keys;
use crate::config::Config;
use crate::vdf_binary::AppInfoEntry;
use crate::vdf_text::Vdf;

#[derive(Debug, Clone)]
pub struct GameMetadata {
    pub app_id: u32,
    pub name: String,
    pub kind: String,
    pub os_list: Vec<String>,
    pub controller_support: Option<String>,
    pub category_ids: Vec<u32>,
    pub store_tag_ids: Vec<u32>,
    pub genre_ids: Vec<u32>,
    pub review_percentage: Option<u32>,
    pub metacritic_score: Option<u32>,
    pub release_unix: Option<u64>,
    pub deck_category: Option<u32>,
}

pub fn metadata_of(entry: &AppInfoEntry, config: &Config) -> Option<GameMetadata> {
    let common = entry.root.path(&[keys::APP_INFO, keys::COMMON])?;
    Some(GameMetadata {
        app_id: entry.app_id,
        name: common.text_at(&[keys::NAME])?.to_string(),
        kind: common
            .text_at(&[keys::KIND])
            .unwrap_or_default()
            .to_string(),
        os_list: split_list(common.text_at(&[keys::OS_LIST]), &config.os_list_separator),
        controller_support: common
            .text_at(&[keys::CONTROLLER_SUPPORT])
            .map(str::to_string),
        category_ids: category_ids(common),
        store_tag_ids: values_as_numbers(common.get(keys::STORE_TAGS)),
        genre_ids: values_as_numbers(common.get(keys::GENRES)),
        review_percentage: common
            .number_at(&[keys::REVIEW_PERCENTAGE])
            .map(|v| v as u32),
        metacritic_score: common
            .number_at(&[keys::METACRITIC_SCORE])
            .map(|v| v as u32),
        release_unix: common.number_at(&[keys::RELEASE_DATE]),
        deck_category: common
            .number_at(&[keys::DECK_COMPATIBILITY, keys::DECK_CATEGORY])
            .map(|v| v as u32),
    })
}

fn split_list(value: Option<&str>, separator: &str) -> Vec<String> {
    value
        .map(|text| text.split(separator).map(str::to_string).collect())
        .unwrap_or_default()
}

fn category_ids(common: &Vdf) -> Vec<u32> {
    let Some(categories) = common.get(keys::CATEGORY) else {
        return Vec::new();
    };
    categories
        .entries()
        .iter()
        .filter_map(|(key, _)| key.strip_prefix(keys::CATEGORY_KEY_PREFIX)?.parse().ok())
        .collect()
}

fn values_as_numbers(node: Option<&Vdf>) -> Vec<u32> {
    let Some(node) = node else {
        return Vec::new();
    };
    node.entries()
        .iter()
        .filter_map(|(_, value)| value.text()?.parse().ok())
        .collect()
}
