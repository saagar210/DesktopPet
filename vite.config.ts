import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";
import { resolve } from "path";

const host = process.env.TAURI_DEV_HOST;
const leanCacheDir = process.env.DESKTOPPET_VITE_CACHE_DIR;

export default defineConfig({
  plugins: [react(), tailwindcss()],
  clearScreen: false,
  cacheDir: leanCacheDir || "node_modules/.vite",
  server: {
    port: 5173,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: "ws", host, port: 5174 } : undefined,
    watch: { ignored: ["**/src-tauri/**"] },
  },
  build: {
    rollupOptions: {
      input: {
        pet: resolve(__dirname, "pet.html"),
        panel: resolve(__dirname, "panel.html"),
      },
    },
  },
});
