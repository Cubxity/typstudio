use super::Error;
use super::Result;
use crate::ipc::commands::project_path;
use crate::project::ProjectManager;
use arboard::Clipboard;
use chrono::Local;
use log::info;
use serde::Serialize;
use std::fs;
use std::fs::File;
use std::io::BufWriter;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::Runtime;

#[derive(Serialize, Debug)]
pub struct ClipboardPasteResponse {
    path: PathBuf,
}

#[tauri::command]
pub async fn clipboard_paste<R: Runtime>(
    window: tauri::Window<R>,
    project_manager: tauri::State<'_, Arc<ProjectManager<R>>>,
) -> Result<ClipboardPasteResponse> {
    let now = Local::now();
    let (_, path) = project_path(&window, &project_manager, PathBuf::from("assets"))?;

    let now_format = now.format("%Y-%m-%d %H:%M:%S.png");

    fs::create_dir_all(&path).map_err(Into::<Error>::into)?;
    let path = path.join(now_format.to_string());

    // TODO: Better error handling
    let mut clipboard = Clipboard::new().map_err(|_| Error::Unknown)?;
    let data = clipboard.get_image().map_err(|_| Error::Unknown)?;

    let file = File::create(&path).map_err(Into::<Error>::into)?;
    let ref mut w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, data.width as u32, data.height as u32);
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().map_err(|_| Error::Unknown)?;
    writer
        .write_image_data(&*data.bytes)
        .map_err(|_| Error::Unknown)?;

    info!(
        "wrote {}x{} image from clipboard to {:?}",
        data.width, data.height, path
    );
    Ok(ClipboardPasteResponse {
        path: PathBuf::from(format!("assets/{}", now_format)),
    })
}
