use crate::project::{Project, ProjectManager, ProjectWorld};
use std::fs;
use std::sync::{Arc, RwLock};
use tauri::api::dialog::FileDialogBuilder;
use tauri::{Manager, Runtime, State, WindowMenuEvent};

pub fn handle_menu_event<R: Runtime>(e: WindowMenuEvent<R>) {
    match e.menu_item_id() {
        "file_open_project" => FileDialogBuilder::new()
            .set_title("Open Project")
            .pick_folder(move |path| {
                if let Some(path) = path {
                    let path = fs::canonicalize(&path).unwrap_or(path);

                    let window = e.window();
                    let project_manager: State<'_, Arc<ProjectManager<_>>> = window.state();
                    let project = Arc::new(Project {
                        world: ProjectWorld::new(path.clone()).into(),
                        cache: RwLock::new(Default::default()),
                        root: path,
                    });
                    project_manager.set_project(window, Some(project));
                }
            }),
        "file_export_pdf" => FileDialogBuilder::new()
            .set_title("Export PDF")
            .set_file_name("export.pdf")
            .save_file(move |path| {
                if let Some(mut path) = path {
                    path.set_extension("pdf");

                    let window = e.window();
                    let project_manager: State<'_, Arc<ProjectManager<_>>> = window.state();
                    if let Some(project) = project_manager.get_project(window) {
                        let cache = project.cache.read().unwrap();
                        if let Some(doc) = &cache.document {
                            let pdf = typst::export::pdf(doc);
                            let _ = fs::write(path, pdf);
                        }
                    }
                }
            }),
        "file_quit" => {
            e.window().app_handle().exit(0);
        }
        _ => {}
    }
}
