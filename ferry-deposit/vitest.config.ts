import { defineConfig } from 'vitest/config';
import dotenv from 'dotenv';
import { resolve } from 'path';

// Load custom .env.test file
dotenv.config({ path: resolve(__dirname, 'env.test') });

export default defineConfig({
  test: {
    // Your vitest configurations
  },
});

