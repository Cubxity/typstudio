import { invoke } from "@tauri-apps/api";

export interface TypstCompileEvent {
  pages: number;
  hash: string;
  width: number;
  height: number;
}

export interface TypstRenderResponse {
  image: string;
}

export const render = (page: number, scale: number): Promise<TypstRenderResponse> =>
  invoke<TypstRenderResponse>("typst_render", { page, scale });
