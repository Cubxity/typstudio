use super::{Error, Result};
use crate::ipc::commands::project_path;
use crate::ipc::model::TypstRenderResponse;
use crate::project::ProjectManager;
use base64::Engine;
use serde::Serialize;
use serde_repr::Serialize_repr;
use std::path::PathBuf;
use std::sync::Arc;
use tauri::Runtime;
use typst::geom::Color;
use typst::ide::{Completion, CompletionKind};
use typst::World;

#[derive(Serialize_repr, Debug)]
#[repr(u8)]
pub enum TypstCompletionKind {
    Syntax = 1,
    Function = 2,
    Parameter = 3,
    Constant = 4,
    Symbol = 5,
}

#[derive(Serialize, Debug)]
pub struct TypstCompletion {
    kind: TypstCompletionKind,
    label: String,
    apply: Option<String>,
    detail: Option<String>,
}

#[derive(Serialize, Debug)]
pub struct TypstCompleteResponse {
    offset: usize,
    completions: Vec<TypstCompletion>,
}

impl From<Completion> for TypstCompletion {
    fn from(value: Completion) -> Self {
        Self {
            kind: match value.kind {
                CompletionKind::Syntax => TypstCompletionKind::Syntax,
                CompletionKind::Func => TypstCompletionKind::Function,
                CompletionKind::Param => TypstCompletionKind::Parameter,
                CompletionKind::Constant => TypstCompletionKind::Constant,
                CompletionKind::Symbol(_) => TypstCompletionKind::Symbol,
            },
            label: value.label.to_string(),
            apply: value.apply.map(|s| s.to_string()),
            detail: value.detail.map(|s| s.to_string()),
        }
    }
}

#[tauri::command]
pub async fn typst_render<R: Runtime>(
    window: tauri::Window<R>,
    project_manager: tauri::State<'_, Arc<ProjectManager<R>>>,
    page: usize,
    scale: f32,
) -> Result<TypstRenderResponse> {
    println!("rendering page: {}", page);
    let project = project_manager
        .get_project(&window)
        .ok_or(Error::UnknownProject)?;

    let cache = project.cache.read().unwrap();
    if let Some(frame) = cache.document.as_ref().and_then(|doc| doc.pages.get(page)) {
        let bmp = typst::export::render(frame, scale, Color::WHITE);
        if let Ok(image) = bmp.encode_png() {
            println!("render complete for page: {}", page);
            let b64 = base64::engine::general_purpose::STANDARD.encode(image);
            return Ok(TypstRenderResponse {
                image: b64,
                width: bmp.width(),
                height: bmp.height(),
            });
        }
    }

    Err(Error::Unknown)
}

#[tauri::command]
pub async fn typst_autocomplete<R: Runtime>(
    window: tauri::Window<R>,
    project_manager: tauri::State<'_, Arc<ProjectManager<R>>>,
    path: PathBuf,
    content: String,
    offset: usize,
    explicit: bool,
) -> Result<TypstCompleteResponse> {
    let (project, path) = project_path(&window, &project_manager, path)?;
    let mut world = project.world.lock().unwrap();

    // TODO: Improve error typing
    let source_id = world
        .slot_update(&*path, Some(content))
        .map_err(Into::<Error>::into)?;
    let source = world.source(source_id);

    let (offset, completions) = typst::ide::autocomplete(&*world, &[], source, offset, explicit)
        .ok_or_else(|| Error::Unknown)?;

    Ok(TypstCompleteResponse {
        offset,
        completions: completions.into_iter().map(TypstCompletion::from).collect(),
    })
}
