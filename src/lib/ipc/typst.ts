import { invoke } from "@tauri-apps/api";

export interface TypstCompileEvent {
  document: TypstDocument | null;
  errors: TypstSourceError[] | null;
}

export interface TypstDocument {
  pages: number;
  hash: string;
  width: number;
  height: number;
}

export interface TypstSourceError {
  range: { start: number; end: number };
  message: string;
}

export interface TypstRenderResponse {
  image: string;
  width: number;
  height: number;
}

export const render = (page: number, scale: number): Promise<TypstRenderResponse> =>
  invoke<TypstRenderResponse>("typst_render", { page, scale });
