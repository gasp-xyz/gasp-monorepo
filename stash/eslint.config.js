import globals from 'globals'
import ts from '@typescript-eslint/eslint-plugin'
import tsParser from '@typescript-eslint/parser'
import importPlugin from 'eslint-plugin-import'
import simpleImportSort from 'eslint-plugin-simple-import-sort'
import prettierPlugin from 'eslint-plugin-prettier'

export default [
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
        ecmaVersion: 'latest',
        sourceType: 'module',
        ecmaFeatures: {
          modules: true,
          importAttributes: true,
        },
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
      "prettier/prettier": ["warn", {
        "importAttributes": "with"
      }] ,
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
