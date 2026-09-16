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
  build: {
    rollupOptions: {
      input: {
        main: resolve(siteDirectory, 'index.html'),
        de: resolve(siteDirectory, 'de/index.html'),
        es: resolve(siteDirectory, 'es/index.html'),
        fr: resolve(siteDirectory, 'fr/index.html'),
        it: resolve(siteDirectory, 'it/index.html'),
        'pt-BR': resolve(siteDirectory, 'pt-BR/index.html'),
        ru: resolve(siteDirectory, 'ru/index.html'),
        'zh-Hans': resolve(siteDirectory, 'zh-Hans/index.html'),
        ja: resolve(siteDirectory, 'ja/index.html'),
      },
    },
  },
});
