import { invoke } from "@tauri-apps/api";

export interface TypstCompileEvent {
  document: TypstDocument | null;
  diagnostics: TypstSourceDiagnostic[] | null;
}

export interface TypstDocument {
  pages: number;
  hash: string;
  width: number;
  height: number;
}

export type TypstDiagnosticSeverity = "error" | "warning";

export interface TypstSourceDiagnostic {
  range: { start: number; end: number };
  severity: TypstDiagnosticSeverity;
  message: string;
  hints: string[];
}

export interface TypstRenderResponse {
  image: string;
  width: number;
  height: number;
  nonce: number;
}

export enum TypstCompletionKind {
  Syntax = 1,
  Function = 2,
  Parameter = 3,
  Constant = 4,
  Symbol = 5,
  Type = 6,
}

export interface TypstCompletion {
  kind: TypstCompletionKind;
  label: string;
  apply: string | null;
  detail: string | null;
}

export interface TypstCompleteResponse {
  offset: number;
  completions: TypstCompletion[];
}

export const compile = (path: string, content: string): Promise<TypstRenderResponse> =>
  invoke<TypstRenderResponse>("typst_compile", { path, content });

export const render = (page: number, scale: number, nonce: number): Promise<TypstRenderResponse> =>
  invoke<TypstRenderResponse>("typst_render", { page, scale, nonce });

export const autocomplete = (
  path: string,
  content: string,
  offset: number,
  explicit: boolean
): Promise<TypstCompleteResponse> =>
  invoke<TypstCompleteResponse>("typst_autocomplete", { path, content, offset, explicit });
