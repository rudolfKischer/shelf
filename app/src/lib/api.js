import { invoke, convertFileSrc } from "@tauri-apps/api/core";

const COMMAND_LOAD_CATALOG = "load_catalog";
const COMMAND_REPORT_READY = "report_ready";

export function loadCatalog() {
  return invoke(COMMAND_LOAD_CATALOG);
}

export function reportReady(label) {
  return invoke(COMMAND_REPORT_READY, { label });
}

export function artUrl(reference) {
  if (reference?.local) return convertFileSrc(reference.local);
  return reference?.remote ?? null;
}
