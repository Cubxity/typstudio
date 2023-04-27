#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod engine;
mod ipc;
mod menu;
mod project;

use crate::menu::handle_menu_event;
use crate::project::ProjectManager;
use env_logger::Env;
use log::info;
use std::sync::Arc;
use tauri::{CustomMenuItem, Menu, Submenu, Wry};

#[tokio::main]
async fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    info!("initializing typstudio");

    let project_manager = Arc::new(ProjectManager::<Wry>::new());
    if let Ok(watcher) = ProjectManager::init_watcher(project_manager.clone()) {
        let _ = project_manager.set_watcher(watcher);
    }

    tauri::Builder::default()
        .menu(build_menu())
        .on_menu_event(handle_menu_event)
        .manage(project_manager)
        .invoke_handler(tauri::generate_handler![
            ipc::commands::fs_list_dir,
            ipc::commands::fs_read_file_binary,
            ipc::commands::fs_read_file_text,
            ipc::commands::fs_create_file,
            ipc::commands::fs_write_file_binary,
            ipc::commands::fs_write_file_text,
            ipc::commands::typst_compile,
            ipc::commands::typst_render,
            ipc::commands::typst_autocomplete,
            ipc::commands::clipboard_paste,
            ipc::commands::fs_create_folder
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn build_menu() -> Menu {
    let file_submenu = Submenu::new(
        "File",
        Menu::new()
            .add_item(CustomMenuItem::new("file_open_project", "Open Project"))
            .add_submenu(Submenu::new(
                "Export",
                Menu::new().add_item(CustomMenuItem::new("file_export_pdf", "Export PDF")),
            ))
            .add_item(CustomMenuItem::new("file_quit", "Quit")),
    );
    let edit_submenu = Submenu::new("Edit", Menu::new());
    let view_submenu = Submenu::new("View", Menu::new());

    Menu::new()
        .add_submenu(file_submenu)
        .add_submenu(edit_submenu)
        .add_submenu(view_submenu)
}
