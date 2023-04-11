use crate::project::{Project, ProjectManager, ProjectWorld};
use std::sync::{Arc, RwLock};
use tauri::api::dialog::FileDialogBuilder;
use tauri::{Manager, Runtime, State, WindowMenuEvent};

pub fn handle_menu_event<R: Runtime>(e: WindowMenuEvent<R>) {
    match e.menu_item_id() {
        "file_open_project" => FileDialogBuilder::new()
            .set_title("Open Project")
            .pick_folder(move |path| {
                if let Some(path) = path {
                    let window = e.window();
                    let project_manager: State<'_, Arc<ProjectManager>> = window.state();
                    let project = Arc::new(Project {
                        root: path,
                        world: ProjectWorld::new().into(),
                        cache: RwLock::new(Default::default()),
                    });
                    project_manager.set_project(window, Some(project));
                }
            }),
        "file_quit" => {
            e.window().app_handle().exit(0);
        }
        _ => {}
    }
}
