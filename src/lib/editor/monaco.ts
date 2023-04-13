import * as monaco from "monaco-editor";

import { tokensProvider } from "./language";

monaco.languages.register({ id: "typst" });
monaco.languages.setMonarchTokensProvider("typst", tokensProvider);

export * from "monaco-editor";
