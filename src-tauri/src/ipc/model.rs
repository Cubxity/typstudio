use serde::Serialize;

#[derive(Serialize, Clone, Debug)]
pub struct TypstCompileEvent {
    pub pages: usize,
    pub hash: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct TypstRenderResponse {
    pub image: String,
}

#[derive(Serialize, Clone, Debug)]
pub struct ProjectChangeEvent {
    pub project: Option<ProjectModel>,
}

#[derive(Serialize, Clone, Debug)]
pub struct ProjectModel {
    pub root: String,
}
