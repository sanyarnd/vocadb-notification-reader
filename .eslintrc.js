module.exports = {
  root: true,
  env: {
    node: true
  },
  extends: [
    "plugin:vue/essential",
    "plugin:vue/strongly-recommended",
    "plugin:vue/recommended",

    "eslint:recommended",
    "@vue/typescript/recommended",
    "@vue/prettier",
    "plugin:prettier/recommended",
    "@vue/prettier/@typescript-eslint"
  ],
  overrides: [
    {
      files: ["*.js"],
      rules: {
        "@typescript-eslint/no-var-requires": "off"
      }
    }
  ],
  parserOptions: {
    ecmaVersion: 2020
  },
  rules: {
    "no-console": process.env.NODE_ENV === "production" ? "error" : "off",
    "no-debugger": process.env.NODE_ENV === "production" ? "error" : "off",

    "lines-between-class-members": [
      "error",
      "always",
      { exceptAfterSingleLine: true }
    ],
    "@typescript-eslint/no-use-before-define": "off",
    "vue/html-closing-bracket-newline": "off",
    "vue/multiline-html-element-content-newline": "off",
    "vue/singleline-html-element-content-newline": "off",
    "vue/max-attributes-per-line": "off"
  }
};
