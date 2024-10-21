// eslint.config.cjs
module.exports = {
  ignores: [
    'dist/*',
    'coverage/*',
    '**/*.d.ts',
    '.yarn',
    'vitest.config.ts',
    '**/*.json',
  ],
  extends: ['eslint:recommended', 'plugin:@typescript-eslint/recommended'],
  parser: '@typescript-eslint/parser',
  plugins: ['@typescript-eslint'],
  rules: {
    // Add your custom rules here
  },
}
