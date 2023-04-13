import type { languages as languagesType } from "monaco-editor";

// Reference: https://www.user.tu-berlin.de/laurmaedje/programmable-markup-language-for-typesetting.pdf
export const tokensProvider: languagesType.IMonarchLanguage = {
  // special ::=
  // '\' | '/' | '[' | ']' | '{' | '}' | '#' | '~' | '-' | '.' | ':' |
  // '"' | "'" | '*' | '_' | '`' | '$' | '=' | '<' | '>' | '@'
  control: /[\\/[\]{}#~\-.:"'*_`$=<>@]/,
  escapes: /\\(?:@control)/,
  keywords: [
    "none",
    "auto",
    "true",
    "false",
    "not",
    "and",
    "or",
    "let",
    "set",
    "show",
    "wrap",
    "if",
    "else",
    "for",
    "in",
    "as",
    "while",
    "break",
    "continue",
    "return",
    "import",
    "include",
    "from",
  ],

  tokenizer: {
    root: [
      [/^\s*=+\s+.+$/, "keyword"],

      // list (starting with * or number)
      [/^\s*[-+]\s/, "keyword"],

      // markup within lines
      { include: "@linecontent" },
    ],
    linecontent: [
      // escapes
      [/@escapes/, "string.escape"],

      // various markup
      [/\b_[^_]+_\b/, "emphasis"],
      [/\*([^\\*]|@escapes)+\*/, "strong"],
      [/`([^\\`]|@escapes)+`/, "variable"],

      // or math
      [/\$/, "delimiter", "@math"],

      // or code
      // [/#/, "delimiter", "@inlinecode"],

      // comments
      [/\/\*/, "comment", "@comment"],
      [/\/\/.*$/, "comment"],
    ],
    comment: [
      [/[^/*]+/, "comment"],
      [/\*\//, "comment", "@pop"],
      [/[/*]/, "comment"],
    ],
    math: [
      // Escapes
      [/@escapes/, "string.escape"],

      [/[a-zA-Z]{2,}/, "keyword"],
      // [/[a-zA-Z]/, "variable"],
      [/\d+/, "number"],
      [/"[^"]*"/, "string"],
      [/\/\*/, "comment", "@comment"],
      [/[\^_&/]/, "operators"],

      [/\$/, "delimiter", "@pop"],
    ],
  },
};
