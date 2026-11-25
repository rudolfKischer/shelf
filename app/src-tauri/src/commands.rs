use std::sync::OnceLock;
use std::time::Instant;

use shelf_core::catalog::{build_catalog, Catalog};
use shelf_core::config::Config;

static PROCESS_START: OnceLock<Instant> = OnceLock::new();

pub fn mark_process_start() {
    PROCESS_START.get_or_init(Instant::now);
}

#[tauri::command]
pub async fn load_catalog() -> Result<Catalog, String> {
    tauri::async_runtime::spawn_blocking(|| build_catalog(&Config::load_default()))
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub fn report_ready(label: String) {
    let elapsed_ms = PROCESS_START
        .get()
        .map(|start| start.elapsed().as_millis())
        .unwrap_or_default();
    eprintln!("[timing] {label} {elapsed_ms} ms");
}
