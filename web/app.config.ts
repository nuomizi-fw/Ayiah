import { defineConfig } from "@solidjs/start/config";
import tanstackRouter from "@tanstack/router-plugin/vite";
import tailwindcss from '@tailwindcss/vite'

export default defineConfig({
  vite: {
    plugins: [tanstackRouter({ target: "solid", quoteStyle: "double" }) as any, tailwindcss()],
  }
});
