use super::{Error, Result};
use crate::ipc::commands::project_path;
use crate::ipc::model::TypstCompileEvent;
use crate::ipc::{TypstDocument, TypstSourceError};
use crate::project::ProjectManager;
use enumset::EnumSetType;
use serde::Serialize;
use siphasher::sip128::{Hasher128, SipHasher};
use std::fs;
use std::fs::{File, OpenOptions};
use std::hash::Hash;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::{Runtime, State, Window};
use typst::syntax::ErrorPos;
use typst::World;

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
    fs::read(&path).map_err(Into::into)
}

#[tauri::command]
pub async fn fs_read_file_text<R: Runtime>(
    window: Window<R>,
    project_manager: State<'_, Arc<ProjectManager<R>>>,
    path: PathBuf,
) -> Result<String> {
    let (_, path) = project_path(&window, &project_manager, path)?;
    fs::read_to_string(&path).map_err(Into::into)
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
    fs::write(&path, content).map_err(Into::into)
}

#[tauri::command]
pub async fn fs_write_file_text<R: Runtime>(
    window: Window<R>,
    project_manager: State<'_, Arc<ProjectManager<R>>>,
    path: PathBuf,
    content: String,
) -> Result<()> {
    let (project, path) = project_path(&window, &project_manager, path)?;
    let _ = File::create(&path)
        .map(|mut f| f.write_all(content.as_bytes()))
        .map_err(Into::<Error>::into)?;

    // TODO: Move this logic somewhere else
    let mut world = project.world.lock().unwrap();
    let source_id = world
        .slot_update(path.as_path(), Some(content))
        .expect("Update failed");
    world.set_main(source_id);

    println!("compiling: {:?}", path);
    match typst::compile(&*world) {
        Ok(doc) => {
            let pages = doc.pages.len();

            let mut hasher = SipHasher::new();
            doc.hash(&mut hasher);
            let hash = hex::encode(hasher.finish128().as_bytes());

            // Assume all pages have the same size
            // TODO: Improve this?
            let first_page = &doc.pages[0];
            let width = first_page.width();
            let height = first_page.height();

            project.cache.write().unwrap().document = Some(doc);

            let _ = window.emit(
                "typst_compile",
                TypstCompileEvent {
                    document: Some(TypstDocument {
                        pages,
                        hash,
                        width: width.to_pt(),
                        height: height.to_pt(),
                    }),
                    errors: None,
                },
            );
        }
        Err(errors) => {
            println!("compile error: {:?}", errors);

            let source = world.source(source_id);
            let errors: Vec<TypstSourceError> = errors
                .iter()
                .filter(|e| e.span.source() == source_id)
                .map(|e| {
                    let span = source.range(e.span);
                    let range = match e.pos {
                        ErrorPos::Full => span,
                        ErrorPos::Start => span.start..span.start,
                        ErrorPos::End => span.end..span.end,
                    };
                    let message = e.message.to_string();
                    TypstSourceError { range, message }
                })
                .collect();

            let _ = window.emit(
                "typst_compile",
                TypstCompileEvent {
                    document: None,
                    errors: Some(errors),
                },
            );
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn fs_list_dir<R: Runtime>(
    window: Window<R>,
    project_manager: State<'_, Arc<ProjectManager<R>>>,
    path: PathBuf,
) -> Result<Vec<FileItem>> {
    let (_, path) = project_path(&window, &project_manager, path)?;
    let list = fs::read_dir(&path).map_err(Into::<Error>::into)?;

    let mut files: Vec<FileItem> = vec![];
    for entry in list {
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
    }

    Ok(files)
}
