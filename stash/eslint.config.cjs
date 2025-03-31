const globals = require('globals')
const ts = require('@typescript-eslint/eslint-plugin')
const tsParser = require('@typescript-eslint/parser')
const importPlugin = require('eslint-plugin-import')
const simpleImportSort = require('eslint-plugin-simple-import-sort')
const prettierPlugin = require('eslint-plugin-prettier')

module.exports = [
  // 1. Global Ignores: Apply to everything
  {
    ignores: [
      'dist/**/*',
      'coverage/**/*',
      '**/*.d.ts',
      '.yarn/**/*',
      'vitest.config.ts',
      '**/*.json',
    ],
  },

  // 2. Base Configuration for all TS/TSX files
  {
    files: ['**/*.ts', '**/*.tsx'], // Apply this block to all TS/TSX files
    languageOptions: {
      sourceType: 'module',
      ecmaVersion: 'latest',
      parserOptions: {
        project: ['tsconfig.eslint.json'],
      },
      globals: {
        ...globals.es2021,
        ...globals.node,
      },
      parser: tsParser,
    },
    plugins: {
      '@typescript-eslint': ts,
      import: importPlugin,
      'simple-import-sort': simpleImportSort,
      prettier: prettierPlugin,
    },
    settings: {
      'import/resolver': {
        typescript: {
          project: 'tsconfig.json',
        },
        node: true,
      },
    },
    rules: {
      // Common rules for all TS/TSX files
      ...ts.configs.recommended.rules,
      'prettier/prettier': 'error',
      '@typescript-eslint/no-var-requires': 'off',
      '@typescript-eslint/explicit-function-return-type': 'off',
      '@typescript-eslint/no-explicit-any': 'off',
      '@typescript-eslint/no-inferrable-types': [
        'warn',
        {
          ignoreParameters: true,
        },
      ],
      '@typescript-eslint/no-unused-vars': 'warn',
      'import/no-unresolved': 'error',
      'simple-import-sort/imports': 'warn',
      'simple-import-sort/exports': 'warn',
    },
  },

  // 3. Specific Overrides for Test Files
  {
    files: ['test/**/*.ts', 'test/**/*.tsx'], // Apply this block ONLY to test files
    rules: {
      // Override specific rules for tests
      '@typescript-eslint/no-unused-expressions': 'off',
      // Add any other test-specific rule overrides here
    },
  },
]
