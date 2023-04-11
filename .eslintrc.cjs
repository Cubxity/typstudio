module.exports = {
  root: true,
  parser: "@typescript-eslint/parser",
  extends: ["eslint:recommended", "plugin:@typescript-eslint/recommended", "prettier", "plugin:import/recommended"],
  plugins: ["svelte3", "@typescript-eslint"],
  ignorePatterns: ["*.cjs"],
  overrides: [{ files: ["*.svelte"], processor: "svelte3/svelte3" }],
  settings: {
    "svelte3/typescript": () => require("typescript")
  },
  parserOptions: {
    sourceType: "module",
    ecmaVersion: 2020,
    project: "./tsconfig.json"
  },
  rules: {
    "import/order": [
      "warn",
      {
        "alphabetize": {
          "order": "asc"
        },
        "newlines-between": "always"
      }
    ],
    "@typescript-eslint/consistent-type-imports": "error",
    "@typescript-eslint/consistent-type-exports": [
      "error",
      {
        "fixMixedExportsWithInlineTypeSpecifier": true
      }
    ]
  },
  env: {
    browser: true,
    es2017: true,
    node: true
  }
};
