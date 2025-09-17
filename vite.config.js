import { defineConfig } from "vite";
import vue from "@vitejs/plugin-vue";
import { resolve } from "path";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [vue()],

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1421,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      ignored: ["**/src-tauri/**"],
    },
  },
  // 配置路径别名
  resolve: {
    alias: {
      "@": resolve(__dirname, "src"),
    },
  },
  // 解决 timeout: global 错误
  define: {
    global: "globalThis",
  },
  build: {
    // 增加超时时间
    rollupOptions: {
      external: [],
      output: {
        manualChunks: undefined,
      },
    },
    // 构建超时设置
    chunkSizeWarningLimit: 1500,
  },
}));
