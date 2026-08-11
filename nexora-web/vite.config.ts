import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  // Tauri 开发时前端默认端口；与后续 tauri.conf.json 约定保持一致
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
  },
});
