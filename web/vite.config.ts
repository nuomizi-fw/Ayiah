import { defineConfig } from 'vite';
import solidPlugin from 'vite-plugin-solid';
import devtools from 'solid-devtools/vite';
import tailwindcss from '@tailwindcss/vite'
// import { tanstackRouter } from '@tanstack/router-plugin/vite'

export default defineConfig({
  // tanstackRouter({
  //   target: 'solid',
  //   autoCodeSplitting: true,
  // }),
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
  optimizeDeps: {
    exclude: ['@ark-ui/solid'],
  },
  build: {
    target: 'esnext',
  },
});
