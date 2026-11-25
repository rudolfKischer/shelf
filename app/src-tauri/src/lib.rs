mod commands;

pub fn run() {
    commands::mark_process_start();
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            commands::load_catalog,
            commands::report_ready
        ])
        .run(tauri::generate_context!())
        .expect("the application failed to start");
}
