import { invoke } from "@tauri-apps/api";

import type { Project } from "../stores";

export interface FileItem {
  name: string;
  type: FileType;
}

export type FileType = "file" | "directory";

export interface FSRefreshEvent {
  path: string;
}

export interface ProjectChangeEvent {
  project: Project | null;
}

export const readFileBinary = (path: string): Promise<Uint8Array> =>
  invoke<number[]>("fs_read_file_binary", { path }).then((arr) => Uint8Array.from(arr));

export const readFileText = (path: string): Promise<string> =>
  invoke<string>("fs_read_file_text", { path });

export const createFile = (path: string): Promise<never> => invoke("fs_create_file", { path });
export const createFolder = (path: string): Promise<never> => invoke("fs_create_folder", { path });
export const deleteByPath = (path: string): Promise<never> => invoke("fs_delete", { path });


export const writeFileText = (path: string, content: string): Promise<string> =>
  invoke("fs_write_file_text", { path, content });

export const listDir = (path: string): Promise<FileItem[]> =>
  invoke<FileItem[]>("fs_list_dir", { path });
