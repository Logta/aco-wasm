import tailwindcss from "@tailwindcss/vite";
import react from "@vitejs/plugin-react";
import path from "path";
import { defineConfig } from "vite";

export default defineConfig({
  plugins: [react(), tailwindcss()],
  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./src"),
    },
  },
  server: {
    host: true,
    port: 5173,
    fs: {
      allow: [".."],
    },
  },
  build: {
    target: "esnext",
    outDir: "dist",
  },
  optimizeDeps: {
    exclude: ["aco_wasm"],
  },
  assetsInclude: ["**/*.wasm"],
});
