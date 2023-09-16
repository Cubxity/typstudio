mod clipboard;
mod fs;
mod typst;

pub use self::typst::*;
pub use clipboard::*;
pub use fs::*;

use crate::project::{Project, ProjectManager};
use ::typst::diag::FileError;
use serde::{Serialize, Serializer};
use std::io;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;
use tauri::{Runtime, State, Window};

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("unknown error")]
    Unknown,
    #[error("unknown project")]
    UnknownProject,
    #[error("io error occurred")]
    IO(#[from] io::Error),
    #[error("typst file error occurred")]
    TypstFile(#[from] FileError),
    #[error("the provided path does not belong to the project")]
    UnrelatedPath,
}

impl Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_ref())
    }
}

pub type Result<T> = std::result::Result<T, Error>;

pub fn project<R: Runtime>(
    window: &Window<R>,
    project_manager: &State<Arc<ProjectManager<R>>>,
) -> Result<Arc<Project>> {
    project_manager
        .get_project(window)
        .ok_or(Error::UnknownProject)
}

/// Retrieves the project and resolves the path. Furthermore,
/// this function will resolve the path relative to project's root
/// and checks whether the path belongs to the project root.
pub fn project_path<R: Runtime, P: AsRef<Path>>(
    window: &Window<R>,
    project_manager: &State<Arc<ProjectManager<R>>>,
    path: P,
) -> Result<(Arc<Project>, PathBuf)> {
    let project = project_manager
        .get_project(window)
        .ok_or(Error::UnknownProject)?;
    let root_len = project.root.as_os_str().len();
    let mut out = project.root.to_path_buf();
    for component in path.as_ref().components() {
        match component {
            Component::Prefix(_) => {}
            Component::RootDir => {}
            Component::CurDir => {}
            Component::ParentDir => {
                out.pop();
                if out.as_os_str().len() < root_len {
                    return Err(Error::UnrelatedPath);
                }
            }
            Component::Normal(_) => out.push(component),
        }
    }
    Ok((project, out))
}
