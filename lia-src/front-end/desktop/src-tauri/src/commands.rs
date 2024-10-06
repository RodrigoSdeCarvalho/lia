use tauri::Emitter;
use tauri::{State, Window};
use serde::{Serialize, Deserialize};
use lia_core::models::command::{NewCommand, UpdateCommand, Command};
use crate::AppState;
use system::SysConfigs;

#[derive(Serialize, Deserialize)]
pub struct CommandDTO {
    pub name: String,
    pub description: Option<String>,
    pub command_text: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateCommandDTO {
    pub name: String,
    pub new_description: Option<String>,
    pub new_command_text: Option<String>,
    pub new_tags: Option<Vec<String>>,
}

#[tauri::command]
pub async fn init_db() -> Result<(), String> {
    lia_core::LiaCore::init().await.map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn add_command(state: State<'_, AppState>, command: CommandDTO) -> Result<(), String> {
    let lia_core = state.lia_core.clone();

    let new_command = NewCommand {
        name: command.name,
        description: command.description,
        command_text: command.command_text,
        tags: command.tags,
    };
    lia_core
        .add_command(new_command)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn update_command(state: State<'_, AppState>, command: UpdateCommandDTO) -> Result<(), String> {
    let lia_core = state.lia_core.clone();

    let update_command = UpdateCommand {
        name: command.name,
        new_description: command.new_description,
        new_command_text: command.new_command_text,
        new_tags: command.new_tags,
    };
    lia_core
        .update_command(update_command)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn list_commands(state: State<'_, AppState>, limit: i64, offset: i64) -> Result<Vec<Command>, String> {
    let lia_core = state.lia_core.clone();

    lia_core
        .get_all_commands(limit, offset)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_commands(
    state: State<'_, AppState>,
    query: Option<String>,
    tags: Option<Vec<String>>,
    limit: i64,
    offset: i64,
) -> Result<Vec<Command>, String> {
    let lia_core = state.lia_core.clone();

    lia_core
        .search_commands(&query.unwrap_or_default(), tags, limit, offset)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn run_command(
    window: Window,
    state: State<'_, AppState>,
    name: String,
    path: String,
) -> Result<(), String> {
    let lia_core = state.lia_core.clone();

    let cmd = lia_core
        .get_command_by_name(&name)
        .await
        .map_err(|e| e.to_string())?;
    let path = std::path::Path::new(&path).to_path_buf();

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();

    // Spawn a task to run the command
    tauri::async_runtime::spawn(async move {
        if let Err(e) = lia_core.run_command_stream(cmd, &path, tx).await {
            eprintln!("Error running command: {}", e);
        }
    });

    // Read from the channel and emit events
    while let Some(line) = rx.recv().await {
        window.emit("log", line).unwrap();
    }

    Ok(())
}

#[tauri::command]
pub async fn delete_commands(
    state: State<'_, AppState>,
    name: Option<String>,
    tags: Option<Vec<String>>,
    all: bool,
) -> Result<(), String> {
    let lia_core = state.lia_core.clone();

    if all {
        lia_core
            .delete_all_commands()
            .await
            .map_err(|e| e.to_string())
    } else {
        lia_core
            .delete_commands(name, tags)
            .await
            .map_err(|e| e.to_string())
    }
}

#[tauri::command]
pub fn set_logging(on: bool) -> Result<(), String> {
    SysConfigs::set_log(on, false, None);
    Ok(())
}
