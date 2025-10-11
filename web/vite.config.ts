import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import devtools from 'solid-devtools/vite';
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
  plugins: [devtools(), solidPlugin(), tailwindcss(),],
  server: {
    port: 3000,
    proxy: {
      '/api': {
        target: 'http://127.0.0.1:7590',
        changeOrigin: true,
      },
    },
  },
  build: {
    target: 'esnext',
  },
});
