#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

mod engine;
mod menu;
mod project;
mod rpc;

use crate::menu::handle_menu_event;
use crate::project::ProjectManager;
use std::sync::Arc;
use tauri::{CustomMenuItem, Menu, Submenu};

fn main() {
    let project_manager = Arc::new(ProjectManager::new());

    tauri::Builder::default()
        .menu(build_menu())
        .on_menu_event(handle_menu_event)
        .manage(project_manager)
        .invoke_handler(tauri::generate_handler![
            rpc::commands::fs_list,
            rpc::commands::fs_read_file,
            rpc::commands::fs_update_file,
            rpc::commands::typst_render
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn build_menu() -> Menu {
    let file_submenu = Submenu::new(
        "File",
        Menu::new()
            .add_item(CustomMenuItem::new("file_open_project", "Open Project"))
            .add_submenu(Submenu::new("Export", Menu::new()))
            .add_item(CustomMenuItem::new("file_quit", "Quit")),
    );
    let edit_submenu = Submenu::new("Edit", Menu::new());
    let view_submenu = Submenu::new("View", Menu::new());

    Menu::new()
        .add_submenu(file_submenu)
        .add_submenu(edit_submenu)
        .add_submenu(view_submenu)
}
