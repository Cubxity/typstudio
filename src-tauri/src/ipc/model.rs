use serde::Serialize;
use std::ops::Range;
use std::path::PathBuf;

#[derive(Serialize, Clone, Debug)]
pub struct TypstCompileEvent {
    pub document: Option<TypstDocument>,
    pub diagnostics: Option<Vec<TypstSourceDiagnostic>>,
}

#[derive(Serialize, Clone, Debug)]
pub struct TypstDocument {
    pub pages: usize,
    pub hash: String,
    pub width: f64,
    pub height: f64,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum TypstDiagnosticSeverity {
    Error,
    Warning,
}

#[derive(Serialize, Clone, Debug)]
pub struct TypstSourceDiagnostic {
    pub range: Range<usize>,
    pub severity: TypstDiagnosticSeverity,
    pub message: String,
    pub hints: Vec<String>,
}

#[derive(Serialize, Clone, Debug)]
pub struct TypstRenderResponse {
    pub image: String,
    pub width: u32,
    pub height: u32,
    pub nonce: u32,
}

#[derive(Serialize, Clone, Debug)]
pub struct ProjectChangeEvent {
    pub project: Option<ProjectModel>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ProjectModel {
    pub root: PathBuf,
}

#[derive(Serialize, Clone, Debug)]
pub struct FSRefreshEvent {
    pub path: PathBuf,
}
