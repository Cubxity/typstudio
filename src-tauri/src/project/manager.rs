use crate::ipc::{FSRefreshEvent, ProjectChangeEvent, ProjectModel};
use crate::project::{is_project_config_file, Project, ProjectConfig};
use log::{debug, error, info, trace, warn};
use notify::event::ModifyKind;
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, RwLock};
use tauri::{Runtime, Window};
use tokio::sync::mpsc::channel;

#[derive(Clone, Copy, Debug)]
enum FSHandleKind {
    Refresh,
    Reload,
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
            root: p.root.clone(),
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
                p.config.read().unwrap().apply(&*p);

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

        info!("project set for window {}: {:?}", window.label(), model);
        let _ = window.emit("project_changed", ProjectChangeEvent { project: model });
    }

    fn handle_fs_event(&self, event: notify::Event) {
        let opt = match event.kind {
            EventKind::Create(_) | EventKind::Remove(_) => event.paths[0]
                .parent()
                .map(|p| (p.to_path_buf(), FSHandleKind::Refresh)),
            EventKind::Modify(kind) => match kind {
                ModifyKind::Name(_) => event.paths[0]
                    .parent()
                    .map(|p| (p.to_path_buf(), FSHandleKind::Refresh)),
                ModifyKind::Data(_) => Some((event.paths[0].clone(), FSHandleKind::Reload)),
                _ => None,
            },
            _ => None,
        };

        if let Some((path, kind)) = opt {
            let path = path.canonicalize().unwrap_or(path);
            let projects = self.projects.read().unwrap();

            for (window, project) in &*projects {
                if path.starts_with(&project.root) {
                    self.handle_project_fs_event(project, window, &path, kind);
                }
            }
        }
    }

    fn handle_project_fs_event(
        &self,
        project: &Project,
        window: &Window<R>,
        path: &PathBuf,
        kind: FSHandleKind,
    ) {
        trace!(
            "handling fs event for {:?} (path: {:?}, kind: {:?})",
            project,
            path,
            kind
        );
        match kind {
            // Refreshes the explorer view
            FSHandleKind::Refresh => {
                if let Ok(relative) = path.strip_prefix(&project.root) {
                    let event = FSRefreshEvent {
                        path: relative.to_path_buf(),
                    };
                    let _ = window.emit("fs_refresh", &event);
                }
            }
            // Reloads the file content, eg. project config or project source files
            FSHandleKind::Reload => {
                if let Ok(relative) = path.strip_prefix(&project.root) {
                    if is_project_config_file(relative) {
                        if let Ok(config) = ProjectConfig::read_from_file(path) {
                            debug!("updating project config for {:?}: {:?}", project, config);
                            let mut config_write = project.config.write().unwrap();
                            *config_write = config;
                            config_write.apply(project);
                        }
                    } else {
                        let mut world = project.world.lock().unwrap();
                        let path = Path::new("/").join(relative);
                        match world.slot_update(&path, None) {
                            Ok(id) => {
                                debug!("updated slot for {:?} {:?} in {:?}", path, id, project);
                            }
                            Err(e) => {
                                warn!(
                                    "unable to update slot for {:?} in {:?}: {:?}",
                                    path, project, e
                                );
                            }
                        }
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
