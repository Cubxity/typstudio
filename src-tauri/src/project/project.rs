use crate::ipc::{FSRefreshEvent, ProjectChangeEvent, ProjectModel};
use crate::project::ProjectWorld;
use log::{error, info};
use notify::event::ModifyKind;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::{Arc, Mutex, RwLock};
use tauri::{Runtime, Window};
use tokio::sync::mpsc::channel;
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

pub struct ProjectManager<R: Runtime> {
    projects: RwLock<HashMap<Window<R>, Arc<Project>>>,
    watcher: Mutex<Option<Box<dyn Watcher + Send + Sync>>>,
}

impl<R: Runtime> ProjectManager<R> {
    pub fn init_watcher(
        project_manager: Arc<ProjectManager<R>>,
    ) -> anyhow::Result<Box<dyn Watcher + Send + Sync>> {
        let (tx, mut rx) = channel(1);

        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?;

        let watcher = RecommendedWatcher::new(
            move |res| {
                let _ = rt.block_on(tx.send(res));
            },
            Config::default(),
        )?;

        tokio::spawn(async move {
            while let Some(res) = rx.recv().await {
                match res {
                    Ok(event) => project_manager.handle_fs_event(event),
                    Err(e) => error!("watch error {:?}", e),
                }
            }
        });

        Ok(Box::new(watcher))
    }

    pub fn set_watcher(&self, watcher: Box<dyn Watcher + Send + Sync>) {
        let mut inner = self.watcher.lock().unwrap();
        *inner = Some(watcher);
    }

    pub fn get_project(&self, window: &Window<R>) -> Option<Arc<Project>> {
        self.projects.read().unwrap().get(window).cloned()
    }

    pub fn set_project(&self, window: &Window<R>, project: Option<Arc<Project>>) {
        let mut projects = self.projects.write().unwrap();
        let model = project.as_ref().map(|p| ProjectModel {
            root: p.root.clone().into_os_string().into_string().unwrap(),
        });
        match project {
            None => {
                if let Some(old) = projects.remove(window) {
                    let mut guard = self.watcher.lock().unwrap();
                    if let Some(watcher) = guard.as_mut() {
                        let _ = watcher.unwatch(&old.root);
                    }
                }
            }
            Some(p) => {
                let root = &p.root.clone();
                let mut guard = self.watcher.lock().unwrap();
                if let Some(old) = projects.insert(window.clone(), p) {
                    if let Some(watcher) = guard.as_mut() {
                        let _ = watcher.unwatch(&old.root);
                    }
                }
                if let Some(watcher) = guard.as_mut() {
                    let _ = watcher.watch(root, RecursiveMode::Recursive);
                }
            }
        };

        info!(
            "project set for window {}: {:?}",
            window.label(),
            model.as_ref().map(|m| &m.root)
        );
        let _ = window.emit("project_changed", ProjectChangeEvent { project: model });
    }

    fn handle_fs_event(&self, event: notify::Event) {
        let path = match event.kind {
            EventKind::Create(_) | EventKind::Remove(_) => {
                event.paths[0].parent().map(|p| p.to_path_buf())
            }
            EventKind::Modify(kind) => match kind {
                ModifyKind::Name(_) => event.paths[0].parent().map(|p| p.to_path_buf()),
                _ => None,
            },
            _ => None,
        };

        if let Some(path) = path {
            let projects = self.projects.read().unwrap();

            for (window, project) in &*projects {
                if path.starts_with(&project.root) {
                    if let Ok(relative) = path.strip_prefix(&project.root) {
                        let event = FSRefreshEvent {
                            path: relative.to_path_buf(),
                        };
                        let _ = window.emit("fs_refresh", &event);
                    }
                }
            }
        }
    }

    pub fn new() -> Self {
        Self {
            projects: RwLock::new(HashMap::new()),
            watcher: Mutex::new(None),
        }
    }
}
