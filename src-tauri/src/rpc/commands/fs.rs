use crate::project::ProjectManager;
use crate::rpc::model::{FileItem, FileType, FsListResponse, RpcError, TypstCompileEvent};
use crate::rpc::FsReadResponse;
use siphasher::sip128::{Hasher128, SipHasher};
use std::fs;
use std::fs::File;
use std::hash::Hash;
use std::io::Write;
use std::sync::Arc;
use tauri::Runtime;

#[tauri::command]
pub async fn fs_read_file<R: Runtime>(
    window: tauri::Window<R>,
    path: String,
    project_manager: tauri::State<'_, Arc<ProjectManager>>,
) -> Result<FsReadResponse, RpcError> {
    if let Some(project) = project_manager.get_project(&window) {
        let p = project.root.join(path);
        let res = fs::read_to_string(p).map_err(|_| RpcError::IOError)?;
        return Ok(FsReadResponse { content: res });
    }
    Err(RpcError::Unknown)
}

#[tauri::command]
pub async fn fs_update_file<R: Runtime>(
    window: tauri::Window<R>,
    path: String,
    content: String,
    project_manager: tauri::State<'_, Arc<ProjectManager>>,
) -> Result<(), ()> {
    if let Some(project) = project_manager.get_project(&window) {
        let p = project.root.join(path);
        println!("updating file: {:?}", p);
        let res = File::create(&p).and_then(|mut f| f.write_all(content.as_bytes()));

        match res {
            Ok(_) => {
                println!("update successful");
            }
            Err(e) => {
                println!("update failed: {:?}", e);
            }
        }

        let mut world = project.world.lock().unwrap();
        let source = world.slot_update(p.as_path()).expect("Update failed");
        world.set_main(source);

        println!("compiling: {:?}", p);
        match typst::compile(&*world) {
            Ok(doc) => {
                println!("compiled: {:?}", doc);
                let pages = doc.pages.len();

                let mut hasher = SipHasher::new();
                doc.hash(&mut hasher);
                let hash = hex::encode(hasher.finish128().as_bytes());

                project.cache.write().unwrap().document = Some(doc);
                let _ = window.emit("typst_compile", TypstCompileEvent { pages, hash });
            }
            Err(e) => {
                println!("compile error: {:?}", e);
            }
        }
    }

    Ok(())
}

#[tauri::command]
pub async fn fs_list<R: Runtime>(
    window: tauri::Window<R>,
    project_manager: tauri::State<'_, Arc<ProjectManager>>,
    path: String,
) -> Result<FsListResponse, RpcError> {
    // TODO: Assure that path does not traverse above project's root directory
    if let Some(project) = project_manager.get_project(&window) {
        let path = project.root.join(path);
        println!("listing {:?}", path);
        let list = fs::read_dir(path).map_err(|_| RpcError::IOError)?;
        let mut files: Vec<FileItem> = vec![];
        for entry in list {
            if let Ok(entry) = entry {
                if let (Ok(file_type), Ok(name)) =
                    (entry.file_type(), entry.file_name().into_string())
                {
                    let t = if file_type.is_dir() {
                        FileType::Directory
                    } else {
                        FileType::File
                    };
                    files.push(FileItem { name, file_type: t });
                }
            }
        }

        return Ok(FsListResponse { files });
    }
    Err(RpcError::Unknown)
}
