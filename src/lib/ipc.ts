import type { Project } from "./stores";

export interface FsListResponse {
  files: FileEntry[];
}

export interface FileEntry {
  name: string;
  type: FileType;
}

export type FileType = "file" | "directory";

export interface FsReadResponse {
  content: string;
}

export interface TypstCompileEvent {
  pages: number;
  hash: string;
}

export interface TypstRenderResponse {
  image: string;
}

export interface ProjectChangeEvent {
  project: Project | null;
}
