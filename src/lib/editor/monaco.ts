import * as monaco from "monaco-editor";
import * as oniguruma from "vscode-oniguruma";
import onigurumaWasm from "vscode-oniguruma/release/onig.wasm?url";
import { Registry } from "vscode-textmate";

import grammar from "./data/grammar.json";
import theme from "./data/theme.json";
import { wireTextMateGrammars } from "./grammar";

export const initMonaco = (async () => {
  // Don't use streaming due to MIME type mismatch.
  // See: https://github.com/tauri-apps/tauri/issues/5749
  // TODO: Switch to streaming once Tauri v2 is out
  const wasm = await fetch(onigurumaWasm).then((res) => res.arrayBuffer());
  await oniguruma.loadWASM(wasm);

  const registry = new Registry({
    onigLib: Promise.resolve(oniguruma),
    loadGrammar() {
      return Promise.resolve(grammar);
    },
  });

  const grammars = new Map();
  grammars.set("typst", "source.typst");

  monaco.languages.register({ id: "typst" });
  await wireTextMateGrammars(registry, { typst: "source.typst" });

  monaco.editor.defineTheme("dracula", theme as monaco.editor.IStandaloneThemeData);
  monaco.editor.setTheme("dracula");
})();
