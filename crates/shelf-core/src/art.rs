use std::fs;
use std::path::{Path, PathBuf};

use serde::Serialize;

use crate::config::Config;
use crate::templates::fill;

#[derive(Debug, Clone, Serialize, Default)]
pub struct ArtRef {
    pub local: Option<String>,
    pub remote: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct ArtSet {
    pub capsule: ArtRef,
    pub hero: ArtRef,
    pub logo: ArtRef,
}

pub fn resolve_art(library_caches: &[PathBuf], app_id: u32, config: &Config) -> ArtSet {
    let app = app_id.to_string();
    let find = |names: &[String]| find_in_caches(library_caches, &app, names);
    ArtSet {
        capsule: ArtRef {
            local: find(&config.art_capsule_files),
            remote: Some(fill(&config.art_cdn_capsule_template, &[("app_id", &app)])),
        },
        hero: ArtRef {
            local: find(&config.art_hero_files),
            remote: Some(fill(&config.art_cdn_hero_template, &[("app_id", &app)])),
        },
        logo: ArtRef {
            local: find(&config.art_logo_files),
            remote: None,
        },
    }
}

fn find_in_caches(library_caches: &[PathBuf], app: &str, names: &[String]) -> Option<String> {
    library_caches
        .iter()
        .filter_map(|cache| find_artwork(&cache.join(app), names))
        .next()
        .map(|path| path.to_string_lossy().into_owned())
}

pub fn find_artwork(folder: &Path, names: &[String]) -> Option<PathBuf> {
    names
        .iter()
        .map(|name| folder.join(name))
        .find(|path| path.is_file())
        .or_else(|| find_in_subfolders(folder, names))
}

fn find_in_subfolders(folder: &Path, names: &[String]) -> Option<PathBuf> {
    let entries = fs::read_dir(folder).ok()?;
    entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.is_dir())
        .find_map(|sub| {
            names
                .iter()
                .map(|name| sub.join(name))
                .find(|path| path.is_file())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn missing_folder_yields_no_local_art_but_keeps_remote_url() {
        let config = Config::load_default();
        let art = resolve_art(&[PathBuf::from("/nonexistent")], 7, &config);
        assert!(art.capsule.local.is_none());
        assert_eq!(
            art.capsule.remote.as_deref(),
            Some("https://cdn.akamai.steamstatic.com/steam/apps/7/library_600x900.jpg")
        );
    }
}
