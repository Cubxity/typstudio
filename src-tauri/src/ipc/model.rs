use serde::Serialize;

#[derive(Serialize)]
pub enum IPCError {
    Unknown,
    IOError,
}

#[derive(Serialize)]
pub struct FsListResponse {
    pub files: Vec<FileItem>,
}

#[derive(Serialize)]
pub struct FsReadResponse {
    pub content: String,
}

#[derive(Serialize)]
pub struct FileItem {
    pub name: String,
    #[serde(rename = "type")]
    pub file_type: FileType,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FileType {
    File,
    Directory,
}

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
