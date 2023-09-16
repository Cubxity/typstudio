use super::{Error, Result};
use crate::ipc::commands::project_path;
use crate::project::ProjectManager;
use enumset::EnumSetType;
use serde::Serialize;
use std::cmp::Ordering;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{Runtime, State, Window};

#[derive(Serialize, Debug)]
pub struct FileItem {
    pub name: String,
    #[serde(rename = "type")]
    pub file_type: FileType,
}

#[derive(EnumSetType, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum FileType {
    File,
    Directory,
}

/// Reads raw bytes from a specified path.
/// Note that this command is slow compared to the text API due to Wry's
/// messaging system in v1. See: https://github.com/tauri-apps/tauri/issues/1817
#[tauri::command]
pub async fn fs_read_file_binary<R: Runtime>(
    window: Window<R>,
    project_manager: State<'_, Arc<ProjectManager<R>>>,
    path: PathBuf,
) -> Result<Vec<u8>> {
    let (_, path) = project_path(&window, &project_manager, path)?;
    fs::read(path).map_err(Into::into)
}

#[tauri::command]
pub async fn fs_read_file_text<R: Runtime>(
    window: Window<R>,
    project_manager: State<'_, Arc<ProjectManager<R>>>,
    path: PathBuf,
) -> Result<String> {
    let (_, path) = project_path(&window, &project_manager, path)?;
    fs::read_to_string(path).map_err(Into::into)
}

#[tauri::command]
pub async fn fs_create_file<R: Runtime>(
    window: Window<R>,
    project_manager: State<'_, Arc<ProjectManager<R>>>,
    path: PathBuf,
) -> Result<()> {
    let (_, path) = project_path(&window, &project_manager, path)?;

    // Not sure if there's a scenario where this condition is not met
    // unless the project is located at `/`
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(Into::<Error>::into)?;
    }
    OpenOptions::new()
        .read(true)
        .write(true)
        .create_new(true)
        .open(&*path)
        .map_err(Into::<Error>::into)?;
    Ok(())
}

#[tauri::command]
pub async fn fs_write_file_binary<R: Runtime>(
    window: Window<R>,
    project_manager: State<'_, Arc<ProjectManager<R>>>,
    path: PathBuf,
    content: Vec<u8>,
) -> Result<()> {
    let (_, path) = project_path(&window, &project_manager, path)?;
    fs::write(path, content).map_err(Into::into)
}

#[tauri::command]
pub async fn fs_write_file_text<R: Runtime>(
    window: Window<R>,
    project_manager: State<'_, Arc<ProjectManager<R>>>,
    path: PathBuf,
    content: String,
) -> Result<()> {
    let (project, absolute_path) = project_path(&window, &project_manager, &path)?;
    let _ = File::create(absolute_path)
        .map(|mut f| f.write_all(content.as_bytes()))
        .map_err(Into::<Error>::into)?;

    let mut world = project.world.lock().unwrap();
    let _ = world
        .slot_update(&path, Some(content))
        .map_err(Into::<Error>::into)?;

    Ok(())
}

#[tauri::command]
pub async fn fs_list_dir<R: Runtime>(
    window: Window<R>,
    project_manager: State<'_, Arc<ProjectManager<R>>>,
    path: PathBuf,
) -> Result<Vec<FileItem>> {
    let (_, path) = project_path(&window, &project_manager, path)?;
    let list = fs::read_dir(path).map_err(Into::<Error>::into)?;

    let mut files: Vec<FileItem> = vec![];
    list.into_iter().for_each(|entry| {
        if let Ok(entry) = entry {
            if let (Ok(file_type), Ok(name)) = (entry.file_type(), entry.file_name().into_string())
            {
                // File should only be directory or file.
                // Symlinks should be resolved in project_path.
                let t = if file_type.is_dir() {
                    FileType::Directory
                } else {
                    FileType::File
                };
                files.push(FileItem { name, file_type: t });
            }
        }
    });

    files.sort_by(|a, b| {
        if a.file_type == FileType::Directory && b.file_type == FileType::File {
            Ordering::Less
        } else if a.file_type == FileType::File && b.file_type == FileType::Directory {
            Ordering::Greater
        } else {
            a.name.cmp(&b.name)
        }
    });

    Ok(files)
}
