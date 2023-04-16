import { invoke } from "@tauri-apps/api";

export interface TypstCompileEvent {
  pages: number;
  hash: string;
}

export interface TypstRenderResponse {
  image: string;
}

export const render = (page: number): Promise<TypstRenderResponse> =>
  invoke<TypstRenderResponse>("typst_render", { page });
