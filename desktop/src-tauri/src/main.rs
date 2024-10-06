#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Arc;

mod commands;
use commands::*;

use lia_core::LiaCore;

struct AppState {
    lia_core: Arc<LiaCore>,
}

fn main() {
    tauri::Builder::default()
        .manage(AppState {
            lia_core: Arc::new(tauri::async_runtime::block_on(LiaCore::new()).expect("Failed to initialize LiaCore")),
        })
        .invoke_handler(tauri::generate_handler![
            init_db,
            add_command,
            update_command,
            list_commands,
            search_commands,
            run_command,
            delete_commands,
            set_logging,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
