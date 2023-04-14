import * as monaco from "monaco-editor";
import * as oniguruma from "vscode-oniguruma";
import onigurumaWasm from "vscode-oniguruma/release/onig.wasm?url";
import { Registry } from "vscode-textmate";

import grammar from "./data/grammar.json";
import theme from "./data/theme.json";
import { wireTextMateGrammars } from "./grammar";

export const initMonaco = (async () => {
  await oniguruma.loadWASM(await fetch(onigurumaWasm));
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
