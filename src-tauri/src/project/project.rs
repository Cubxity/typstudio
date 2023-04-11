use crate::project::ProjectWorld;
use crate::rpc::{ProjectChangeEvent, ProjectModel};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use tauri::{Runtime, Window};
use typst::doc::Document;

pub struct Project {
    pub root: PathBuf,
    pub world: Mutex<ProjectWorld>,
    pub cache: RwLock<ProjectCache>,
}

#[derive(Default)]
pub struct ProjectCache {
    pub document: Option<Document>,
}

pub struct ProjectManager {
    projects: Mutex<HashMap<String, Arc<Project>>>,
}

impl ProjectManager {
    pub fn get_project<R: Runtime>(&self, window: &Window<R>) -> Option<Arc<Project>> {
        self.projects.lock().unwrap().get(window.label()).cloned()
    }

    pub fn set_project<R: Runtime>(&self, window: &Window<R>, project: Option<Arc<Project>>) {
        let mut projects = self.projects.lock().unwrap();
        let model = project.as_ref().map(|p| ProjectModel {
            root: p.root.clone().into_os_string().into_string().unwrap(),
        });
        match project {
            None => projects.remove(window.label()),
            Some(p) => projects.insert(window.label().to_string(), p),
        };
        let _ = window.emit("project_changed", ProjectChangeEvent { project: model });
    }

    pub fn new() -> Self {
        Self {
            projects: Mutex::new(HashMap::new()),
        }
    }
}
