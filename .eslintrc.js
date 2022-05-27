module.exports = {
  root: true,
  extends: [
    'prettier',
    'plugin:@typescript-eslint/eslint-recommended',
    'plugin:@typescript-eslint/recommended',
    'plugin:vue/vue3-recommended',
  ],
  parser: 'vue-eslint-parser',
  parserOptions: {
    parser: '@typescript-eslint/parser',
    sourceType: 'module',
  },
  env: { browser: true, es2021: true, node: true },
  rules: {
    semi: ['warn', 'never'],
  },
}
