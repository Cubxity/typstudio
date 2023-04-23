import { invoke } from "@tauri-apps/api";

export interface ClipboardPasteResponse {
  path: string;
}

export const paste = async (): Promise<ClipboardPasteResponse> =>
  invoke<ClipboardPasteResponse>("clipboard_paste");
