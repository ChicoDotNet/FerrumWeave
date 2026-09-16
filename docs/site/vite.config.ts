import { resolve } from 'node:path';
import { fileURLToPath } from 'node:url';
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

const siteDirectory = fileURLToPath(new URL('.', import.meta.url));
const repository = process.env.GITHUB_REPOSITORY?.split('/')[1];
const base = process.env.GITHUB_ACTIONS && repository ? `/${repository}/` : '/';

export default defineConfig({
  plugins: [react()],
  base,
  publicDir: resolve(siteDirectory, '../../assets/brand'),
});
