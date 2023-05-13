use serde::Serialize;
use tauri::{Runtime, Window};

// For some reason, Tauri requires an event payload...
#[derive(Debug, Clone, Serialize)]
struct EmptyPayload {}

// Instructs the front-end to hide or show the preview
pub fn toggle_preview_visibility<R: Runtime>(window: &Window<R>) {
    let _ = window.emit("toggle_preview_visibility", EmptyPayload {});
}
