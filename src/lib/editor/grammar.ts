import * as monaco from "monaco-editor";
import type { Registry, StateStack } from "vscode-textmate";
import { INITIAL } from "vscode-textmate";

// Wires up monaco-editor with monaco-textmate
// Taken from: https://github.com/microsoft/monaco-editor/discussions/3830
export const wireTextMateGrammars = (
  // TmGrammar `Registry` this wiring should rely on to provide the grammars.
  registry: Registry,
  // `Map` of language ids (string) to TM names (string).
  languages: Record<string, string>
) =>
  Promise.all(
    Array.from(Object.keys(languages)).map(async (languageId) => {
      const grammar = await registry.loadGrammar(languages[languageId]);
      if (!grammar) return;

      monaco.languages.setTokensProvider(languageId, {
        getInitialState: () => new TokenizerState(INITIAL),
        tokenize: (line: string, state: TokenizerState) => {
          const result = grammar.tokenizeLine(line, state.ruleStack);

          return {
            endState: new TokenizerState(result.ruleStack),
            tokens: result.tokens.map((token) => {
              const scopes = token.scopes.slice(0);

              // for (let i = scopes.length - 1; i >= 0; i--) {
              //   const scope = scopes[i];
              //   console.log(scope);
              // const foreground = tokenTheme._match(scope)._foreground;

              // if (foreground !== defaultForeground) {
              // return {
              //   ...token,
              //   scopes: scope,
              // };
              // }
              // }

              return {
                ...token,
                scopes: scopes[scopes.length - 1],
              };
            }),
          };
        },
      });
    })
  );

class TokenizerState implements monaco.languages.IState {
  constructor(private _ruleStack: StateStack) {}

  public get ruleStack(): StateStack {
    return this._ruleStack;
  }

  public clone(): TokenizerState {
    return new TokenizerState(this._ruleStack);
  }

  public equals(other: monaco.languages.IState): boolean {
    return (
      other instanceof TokenizerState && (other === this || other.ruleStack === this.ruleStack)
    );
  }
}
